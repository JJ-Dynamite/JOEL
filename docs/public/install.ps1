#!/usr/bin/env pwsh

param(
  [String]$Version = "latest",
  # Forces installing the baseline build regardless of what CPU you are actually using.
  [Switch]$ForceBaseline = $false,
  # Skips adding the joel.exe directory to the user's %PATH%
  [Switch]$NoPathUpdate = $false,
  # Skips adding the joel to the list of installed programs
  [Switch]$NoRegisterInstallation = $false,
  # Skips installing powershell completions to your profile
  [Switch]$NoCompletions = $false,
  # Debugging: Always download with 'Invoke-RestMethod' instead of 'curl.exe'
  [Switch]$DownloadWithoutCurl = $false
);

# filter out 32 bit + ARM
if (-not ((Get-CimInstance Win32_ComputerSystem)).SystemType -match "x64-based") {
  Write-Output "Install Failed:"
  Write-Output "JOEL for Windows is currently only available for x86 64-bit Windows.`n"
  return 1
}

# This corresponds to .win10_rs5 in build.zig
$MinBuild = 17763;
$MinBuildName = "Windows 10 1809 / Windows Server 2019"
$WinVer = [System.Environment]::OSVersion.Version

if ($WinVer.Major -lt 10 -or ($WinVer.Major -eq 10 -and $WinVer.Build -lt $MinBuild)) {
  Write-Warning "JOEL requires at ${MinBuildName} or newer.`n`nThe install will still continue but it may not work.`n"
  return 1
}

$ErrorActionPreference = "Stop"

# These three environment functions are roughly copied from https://github.com/prefix-dev/pixi/pull/692
# They are used instead of `SetEnvironmentVariable` because of unwanted variable expansions.
function Publish-Env {
  if (-not ("Win32.NativeMethods" -as [Type])) {
    Add-Type -Namespace Win32 -Name NativeMethods -MemberDefinition @"
[DllImport("user32.dll", SetLastError = true, CharSet = CharSet.Auto)]
public static extern IntPtr SendMessageTimeout(
    IntPtr hWnd, uint Msg, UIntPtr wParam, string lParam,
    uint fuFlags, uint uTimeout, out UIntPtr lpdwResult);
"@
  }
  $HWND_BROADCAST = [IntPtr] 0xffff
  $WM_SETTINGCHANGE = 0x1a
  $result = [UIntPtr]::Zero
  [Win32.NativeMethods]::SendMessageTimeout($HWND_BROADCAST,
    $WM_SETTINGCHANGE,
    [UIntPtr]::Zero,
    "Environment",
    2,
    5000,
    [ref] $result
  ) | Out-Null
}

function Write-Env {
  param([String]$Key, [String]$Value)
  $RegisterKey = Get-Item -Path 'HKCU:'
  $EnvRegisterKey = $RegisterKey.OpenSubKey('Environment', $true)
  if ($null -eq $Value) {
    $EnvRegisterKey.DeleteValue($Key)
  } else {
    $RegistryValueKind = if ($Value.Contains('%')) {
      [Microsoft.Win32.RegistryValueKind]::ExpandString
    } elseif ($EnvRegisterKey.GetValue($Key)) {
      $EnvRegisterKey.GetValueKind($Key)
    } else {
      [Microsoft.Win32.RegistryValueKind]::String
    }
    $EnvRegisterKey.SetValue($Key, $Value, $RegistryValueKind)
  }
  Publish-Env
}

function Get-Env {
  param([String] $Key)
  $RegisterKey = Get-Item -Path 'HKCU:'
  $EnvRegisterKey = $RegisterKey.OpenSubKey('Environment')
  $EnvRegisterKey.GetValue($Key, $null, [Microsoft.Win32.RegistryValueOptions]::DoNotExpandEnvironmentNames)
}

# The installation of joel is it's own function so that in the unlikely case the $IsBaseline check fails, we can do a recursive call.
# There are also lots of sanity checks out of fear of anti-virus software or other weird Windows things happening.
function Install-Joel {
  param(
    [string]$Version,
    [bool]$ForceBaseline = $False
  );

  # if a semver is given, we need to adjust it to this format: joel-v0.0.0
  if ($Version -match "^\d+\.\d+\.\d+$") {
    $Version = "joel-v$Version"
  }
  elseif ($Version -match "^v\d+\.\d+\.\d+$") {
    $Version = "joel-$Version"
  }

  $Arch = "x64"
  $IsBaseline = $ForceBaseline

  if (!$IsBaseline) {
    $IsBaseline = !( `
      Add-Type -MemberDefinition '[DllImport("kernel32.dll")] public static extern bool IsProcessorFeaturePresent(int ProcessorFeature);' `
        -Name 'Kernel32' -Namespace 'Win32' -PassThru `
    )::IsProcessorFeaturePresent(40);
  }

  $JoelRoot = if ($env:JOEL_INSTALL) { $env:JOEL_INSTALL } else { "${Home}\.joel" }
  $JoelBin = mkdir -Force "${JoelRoot}\bin"

  try {
    Remove-Item "${JoelBin}\joel.exe" -Force
  } catch [System.Management.Automation.ItemNotFoundException] {
    # ignore
  } catch [System.UnauthorizedAccessException] {
    $openProcesses = Get-Process -Name joel -ErrorAction SilentlyContinue | Where-Object { $_.Path -eq "${JoelBin}\joel.exe" }
    if ($openProcesses.Count -gt 0) {
      Write-Output "Install Failed - An older installation exists and is open. Please close open JOEL processes and try again."
      return 1
    }
    Write-Output "Install Failed - An unknown error occurred while trying to remove the existing installation"
    Write-Output $_
    return 1
  } catch {
    Write-Output "Install Failed - An unknown error occurred while trying to remove the existing installation"
    Write-Output $_
    return 1
  }

  $Target = "joel-windows-$Arch"
  if ($IsBaseline) {
    $Target = "joel-windows-$Arch-baseline"
  }

  $BaseURL = "https://github.com/JJ-Dynamite/JOEL/releases"
  $URL = "$BaseURL/$(if ($Version -eq "latest") { "latest/download" } else { "download/$Version" })/$Target.zip"

  $ZipPath = "${JoelBin}\$Target.zip"

  $DisplayVersion = $(
    if ($Version -eq "latest") { "JOEL" }
    elseif ($Version -eq "canary") { "JOEL Canary" }
    elseif ($Version -match "^joel-v\d+\.\d+\.\d+$") { "JOEL $($Version.Substring(5))" }
    else { "JOEL tag='${Version}'" }
  )

  $null = mkdir -Force $JoelBin
  Remove-Item -Force $ZipPath -ErrorAction SilentlyContinue

  # curl.exe is faster than PowerShell 5's 'Invoke-WebRequest'
  # note: 'curl' is an alias to 'Invoke-WebRequest'. so the exe suffix is required
  if (-not $DownloadWithoutCurl) {
    curl.exe "-#SfLo" "$ZipPath" "$URL" 
  }

  if ($DownloadWithoutCurl -or ($LASTEXITCODE -ne 0)) {
    Write-Warning "The command 'curl.exe $URL -o $ZipPath' exited with code ${LASTEXITCODE}`nTrying an alternative download method..."
    try {
      # Use Invoke-RestMethod instead of Invoke-WebRequest because Invoke-WebRequest breaks on
      # some machines
      Invoke-RestMethod -Uri $URL -OutFile $ZipPath
    } catch {
      Write-Output "Install Failed - could not download $URL"
      Write-Output "The command 'Invoke-RestMethod $URL -OutFile $ZipPath' exited with code ${LASTEXITCODE}`n"
      return 1
    }
  }

  if (!(Test-Path $ZipPath)) {
    Write-Output "Install Failed - could not download $URL"
    Write-Output "The file '$ZipPath' does not exist. Did an antivirus delete it?`n"
    return 1
  }

  try {
    $lastProgressPreference = $global:ProgressPreference
    $global:ProgressPreference = 'SilentlyContinue';
    Expand-Archive "$ZipPath" "$JoelBin" -Force
    $global:ProgressPreference = $lastProgressPreference

    if (!(Test-Path "${JoelBin}\$Target\joel.exe")) {
      # Try alternative structure (direct joel.exe in zip)
      if (Test-Path "${JoelBin}\joel.exe") {
        # Already extracted correctly
      } else {
        throw "The file '${JoelBin}\$Target\joel.exe' or '${JoelBin}\joel.exe' does not exist. Download is corrupt or intercepted Antivirus?`n"
      }
    } else {
      Move-Item "${JoelBin}\$Target\joel.exe" "${JoelBin}\joel.exe" -Force
      Remove-Item "${JoelBin}\$Target" -Recurse -Force -ErrorAction SilentlyContinue
    }
  } catch {
    Write-Output "Install Failed - could not unzip $ZipPath"
    Write-Error $_
    return 1
  }

  Remove-Item $ZipPath -Force -ErrorAction SilentlyContinue

  $JoelRevision = "$(& "${JoelBin}\joel.exe" version 2>&1)"

  if ($LASTEXITCODE -eq 1073741795) { # STATUS_ILLEGAL_INSTRUCTION
    if ($IsBaseline) {
      Write-Output "Install Failed - joel.exe (baseline) is not compatible with your CPU.`n"
      Write-Output "Please open a GitHub issue with your CPU model:`nhttps://github.com/JJ-Dynamite/JOEL/issues/new/choose`n"
      return 1
    }
    Write-Output "Install Failed - joel.exe is not compatible with your CPU. This should have been detected before downloading.`n"
    Write-Output "Attempting to download joel.exe (baseline) instead.`n"
    Install-Joel -Version $Version -ForceBaseline $True
    return 1
  }

  if (($LASTEXITCODE -eq 3221225781) -or ($LASTEXITCODE -eq -1073741515)) # STATUS_DLL_NOT_FOUND
  { 
    Write-Output "Install Failed - You are missing a DLL required to run joel.exe"
    Write-Output "This can be solved by installing the Visual C++ Redistributable from Microsoft:`nSee https://learn.microsoft.com/cpp/windows/latest-supported-vc-redist`nDirect Download -> https://aka.ms/vs/17/release/vc_redist.x64.exe`n`n"
    Write-Output "The command '${JoelBin}\joel.exe version' exited with code ${LASTEXITCODE}`n"
    return 1
  }

  if ($LASTEXITCODE -ne 0) {
    Write-Output "Install Failed - could not verify joel.exe"
    Write-Output "The command '${JoelBin}\joel.exe version' exited with code ${LASTEXITCODE}`n"
    return 1
  }

  try {
    $env:IS_JOEL_AUTO_UPDATE = "1"

    if ($NoCompletions) {
      $env:JOEL_NO_INSTALL_COMPLETIONS = "1"
    }

    # Install completions if available
    $output = "$(& "${JoelBin}\joel.exe" completions 2>&1)"
    if ($LASTEXITCODE -ne 0 -and $LASTEXITCODE -ne 1) {
      # Ignore if completions command doesn't exist (exit code 1)
      if ($LASTEXITCODE -ne 1) {
        Write-Output $output
        Write-Output "Warning - could not install completions"
        Write-Output "The command '${JoelBin}\joel.exe completions' exited with code ${LASTEXITCODE}`n"
      }
    }
  } catch {
    # it is possible on powershell 5 that an error happens, but it is probably fine?
  }

  $env:IS_JOEL_AUTO_UPDATE = $null
  $env:JOEL_NO_INSTALL_COMPLETIONS = $null

  $DisplayVersion = "$(& "${JoelBin}\joel.exe" version 2>&1)"

  $C_RESET = [char]27 + "[0m"
  $C_GREEN = [char]27 + "[1;32m"
  Write-Output "${C_GREEN}JOEL ${DisplayVersion} was installed successfully!${C_RESET}"
  Write-Output "The binary is located at ${JoelBin}\joel.exe`n"

  $hasExistingOther = $false;
  try {
    $existing = Get-Command joel -ErrorAction SilentlyContinue
    if ($existing.Source -ne "${JoelBin}\joel.exe") {
      Write-Warning "Note: Another joel.exe is already in %PATH% at $($existing.Source)`nTyping 'joel' in your terminal will not use what was just installed.`n"
      $hasExistingOther = $true;
    }
  } catch {}

  if (-not $NoRegisterInstallation) {
    $rootKey = $null
    try {
      $RegistryKey = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\JOEL"  
      $rootKey = New-Item -Path $RegistryKey -Force
      New-ItemProperty -Path $RegistryKey -Name "DisplayName" -Value "JOEL" -PropertyType String -Force | Out-Null
      New-ItemProperty -Path $RegistryKey -Name "InstallLocation" -Value "${JoelRoot}" -PropertyType String -Force | Out-Null
      New-ItemProperty -Path $RegistryKey -Name "DisplayIcon" -Value "${JoelBin}\joel.exe" -PropertyType String -Force | Out-Null
    } catch {
      if ($rootKey -ne $null) {
        Remove-Item -Path $RegistryKey -Force -ErrorAction SilentlyContinue
      }
    }
  }

  if(!$hasExistingOther) {
    # Only try adding to path if there isn't already a joel.exe in the path
    $Path = (Get-Env -Key "Path") -split ';'
    if ($Path -notcontains $JoelBin) {
      if (-not $NoPathUpdate) {
        $Path += $JoelBin
        Write-Env -Key 'Path' -Value ($Path -join ';')
        $env:PATH = $Path -join ';'
      } else {
        Write-Output "Skipping adding '${JoelBin}' to the user's %PATH%`n"
      }
    }
    Write-Output "To get started, restart your terminal/editor, then type `"joel`"`n"
  }

  $LASTEXITCODE = 0;
}

Install-Joel -Version $Version -ForceBaseline $ForceBaseline

