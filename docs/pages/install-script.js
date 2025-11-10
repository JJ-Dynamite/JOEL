import { promises as fs } from 'fs'
import path from 'path'

export async function getServerSideProps() {
  try {
    // In Next.js, when running from the docs directory, process.cwd() is the docs directory
    // The public folder is at the same level as pages
    const filePath = path.join(process.cwd(), 'public', 'install.ps1')
    const scriptContent = await fs.readFile(filePath, 'utf8')
    return {
      props: {
        script: scriptContent
      }
    }
  } catch (error) {
    console.error('Error reading install script:', error)
    console.error('Current working directory:', process.cwd())
    console.error('Attempted path:', path.join(process.cwd(), 'public', 'install.ps1'))
    return {
      props: {
        script: `# Error loading install script\n# Error: ${error.message}\n# CWD: ${process.cwd()}\n# Path: ${path.join(process.cwd(), 'public', 'install.ps1')}`
      }
    }
  }
}

export default function InstallScriptPage({ script }) {
  return (
    <div style={{
      maxWidth: '1200px',
      margin: '0 auto',
      padding: '2rem',
      background: '#0a0a0a',
      color: '#ffffff',
      minHeight: '100vh'
    }}>
      <h1 style={{
        fontSize: '2rem',
        marginBottom: '1.5rem',
        background: 'linear-gradient(135deg, var(--joel-text) 0%, var(--joel-pink) 100%)',
        WebkitBackgroundClip: 'text',
        WebkitTextFillColor: 'transparent',
        backgroundClip: 'text'
      }}>
        JOEL Install Script
      </h1>
      <p style={{ marginBottom: '2rem', color: 'var(--joel-text-muted)' }}>
        PowerShell installation script for Windows. Copy and run this script to install JOEL.
      </p>
      <div style={{
        background: 'var(--joel-card)',
        border: '1px solid var(--joel-pink)',
        borderRadius: '16px',
        padding: '1.5rem',
        overflow: 'auto',
        position: 'relative',
        transition: 'background-color 0.3s ease, border-color 0.3s ease'
      }}>
        <pre style={{
          margin: 0,
          fontFamily: 'Monaco, Menlo, "Courier New", monospace',
          fontSize: '0.875rem',
          lineHeight: '1.6',
          color: 'var(--joel-text)',
          whiteSpace: 'pre-wrap',
          wordBreak: 'break-word',
          overflowWrap: 'break-word'
        }}>
          <code>{script}</code>
        </pre>
      </div>
      <div style={{ marginTop: '2rem' }}>
        <a 
          href="/"
          style={{
            color: '#ff6ec7',
            textDecoration: 'none',
            fontSize: '1rem'
          }}
        >
          ← Back to Home
        </a>
      </div>
    </div>
  )
}

