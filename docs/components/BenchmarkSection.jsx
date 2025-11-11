'use client'

import { useState } from 'react'

export default function BenchmarkSection() {
  const [activeTab, setActiveTab] = useState('performance')

  const performanceContent = (
    <>
      <div className="benchmark-title">Compiling 10,000 lines of code</div>
      <div className="benchmark-subtitle">Build time in milliseconds (Linux x64)</div>
      
      <div className="benchmark-list">
        <div className="benchmark-item">
          <div className="benchmark-label">JOEL</div>
          <div className="benchmark-bar-container">
            <div className="benchmark-bar" style={{ width: '100%' }}>269.1 ms</div>
          </div>
          <div className="benchmark-value">269.1 ms</div>
        </div>
        
        <div className="benchmark-item">
          <div className="benchmark-label">Rust</div>
          <div className="benchmark-bar-container">
            <div className="benchmark-bar other" style={{ width: '55%' }}>494.9 ms</div>
          </div>
          <div className="benchmark-value">494.9 ms</div>
        </div>
        
        <div className="benchmark-item">
          <div className="benchmark-label">Go</div>
          <div className="benchmark-bar-container">
            <div className="benchmark-bar other" style={{ width: '47%' }}>571.1 ms</div>
          </div>
          <div className="benchmark-value">571.1 ms</div>
        </div>
        
        <div className="benchmark-item">
          <div className="benchmark-label">Node.js</div>
          <div className="benchmark-bar-container">
            <div className="benchmark-bar other" style={{ width: '17%' }}>1,608 ms</div>
          </div>
          <div className="benchmark-value">1,608 ms</div>
        </div>
      </div>
    </>
  )

  const featuresContent = (
    <>
      <div className="benchmark-title">Key Features</div>
      <div className="benchmark-subtitle">What makes JOEL unique</div>
      
      <div className="benchmark-grid">
        <div className="benchmark-feature-item">
          <div className="benchmark-feature-icon">⚡</div>
          <div className="benchmark-feature-title">Polymodal Execution</div>
          <div className="benchmark-feature-desc">Choose [Interpreted] or [Compiled] from a file header</div>
        </div>
        
        <div className="benchmark-feature-item">
          <div className="benchmark-feature-icon">🌐</div>
          <div className="benchmark-feature-title">Cross-Platform</div>
          <div className="benchmark-feature-desc">Native, WASM, EVM, Solana — one codebase</div>
        </div>
        
        <div className="benchmark-feature-item">
          <div className="benchmark-feature-icon">📦</div>
          <div className="benchmark-feature-title">Gradual Types</div>
          <div className="benchmark-feature-desc">Start dynamic, add types as needed</div>
        </div>
        
        <div className="benchmark-feature-item">
          <div className="benchmark-feature-icon">🔒</div>
          <div className="benchmark-feature-title">Ownership-Lite</div>
          <div className="benchmark-feature-desc">Rust-inspired safety without complexity</div>
        </div>
        
        <div className="benchmark-feature-item">
          <div className="benchmark-feature-icon">🚀</div>
          <div className="benchmark-feature-title">Async/Actor Model</div>
          <div className="benchmark-feature-desc">Built-in concurrency for distributed systems</div>
        </div>
        
        <div className="benchmark-feature-item">
          <div className="benchmark-feature-icon">💻</div>
          <div className="benchmark-feature-title">Modern Syntax</div>
          <div className="benchmark-feature-desc">Clean, readable syntax with pattern matching</div>
        </div>
      </div>
    </>
  )

  const targetsContent = (
    <>
      <div className="benchmark-title">Compilation Targets</div>
      <div className="benchmark-subtitle">One language, multiple platforms</div>
      
      <div className="benchmark-grid">
        <div className="benchmark-feature-item">
          <div className="benchmark-feature-icon">🖥️</div>
          <div className="benchmark-feature-title">Native Binaries</div>
          <div className="benchmark-feature-desc">Linux, macOS, Windows — maximum performance</div>
        </div>
        
        <div className="benchmark-feature-item">
          <div className="benchmark-feature-icon">🌐</div>
          <div className="benchmark-feature-title">WebAssembly (WASM)</div>
          <div className="benchmark-feature-desc">Run in browsers and edge environments</div>
        </div>
        
        <div className="benchmark-feature-item">
          <div className="benchmark-feature-icon">⛓️</div>
          <div className="benchmark-feature-title">EVM (Ethereum)</div>
          <div className="benchmark-feature-desc">Smart contracts on Ethereum and L2s</div>
        </div>
        
        <div className="benchmark-feature-item">
          <div className="benchmark-feature-icon">🔷</div>
          <div className="benchmark-feature-title">Solana</div>
          <div className="benchmark-feature-desc">High-performance blockchain programs</div>
        </div>
      </div>
    </>
  )

  const renderContent = () => {
    switch (activeTab) {
      case 'performance':
        return performanceContent
      case 'features':
        return featuresContent
      case 'targets':
        return targetsContent
      default:
        return performanceContent
    }
  }

  return (
    <div className="hero-right">
      <div className="benchmark-tabs">
        <button
          className={`benchmark-tab ${activeTab === 'performance' ? 'active' : ''}`}
          onClick={() => setActiveTab('performance')}
        >
          Performance
        </button>
        <button
          className={`benchmark-tab ${activeTab === 'features' ? 'active' : ''}`}
          onClick={() => setActiveTab('features')}
        >
          Features
        </button>
        <button
          className={`benchmark-tab ${activeTab === 'targets' ? 'active' : ''}`}
          onClick={() => setActiveTab('targets')}
        >
          Targets
        </button>
      </div>
      
      <div className="benchmark-card">
        {renderContent()}
      </div>
    </div>
  )
}

