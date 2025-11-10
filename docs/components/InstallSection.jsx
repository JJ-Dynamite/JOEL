'use client'

import { useState } from 'react'
import CopyButton from './CopyButton'

export default function InstallSection() {
  const [activeTab, setActiveTab] = useState('linux-macos')

  const linuxMacosCommand = 'curl -fsSL https://joel.val-x.com/api/install | bash'
  const windowsCommand = 'powershell -c "irm https://joel.val-x.com/install.ps1 | iex"'

  const renderContent = () => {
    switch (activeTab) {
      case 'linux-macos':
        return (
          <div className="install-command">
            <code>$ {linuxMacosCommand}</code>
            <CopyButton text={linuxMacosCommand} />
          </div>
        )
      case 'windows':
        return (
          <div className="install-command">
            <code>PS{'>'} {windowsCommand}</code>
            <CopyButton text={windowsCommand} />
          </div>
        )
      default:
        return null
    }
  }

  return (
    <div className="install-section">
      <div className="install-title">Install JOEL</div>
      <div className="platform-tabs">
        <button
          className={`platform-tab ${activeTab === 'linux-macos' ? 'active' : ''}`}
          onClick={() => setActiveTab('linux-macos')}
        >
          Linux & macOS
        </button>
        <button
          className={`platform-tab ${activeTab === 'windows' ? 'active' : ''}`}
          onClick={() => setActiveTab('windows')}
        >
          Windows
        </button>
        <a
          href="/install.ps1"
          className="platform-tab"
          style={{ textDecoration: 'none', display: 'inline-block' }}
        >
          View install script
        </a>
      </div>
      {renderContent()}
    </div>
  )
}

