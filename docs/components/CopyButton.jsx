'use client'

import { useState } from 'react'

export default function CopyButton({ text }) {
  const [copied, setCopied] = useState(false)

  const handleCopy = () => {
    if (typeof navigator !== 'undefined' && navigator.clipboard) {
      navigator.clipboard.writeText(text)
      setCopied(true)
      setTimeout(() => {
        setCopied(false)
      }, 2000)
    }
  }

  return (
    <button className="copy-btn" onClick={handleCopy}>
      <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M5.5 3.5V1.5C5.5 1.22386 5.72386 1 6 1H12.5C12.7761 1 13 1.22386 13 1.5V8.5C13 8.77614 12.7761 9 12.5 9H10.5" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round"/>
        <path d="M3.5 5.5H10.5C10.7761 5.5 11 5.72386 11 6V13C11 13.2761 10.7761 13.5 10.5 13.5H3.5C3.22386 13.5 3 13.2761 3 13V6C3 5.72386 3.22386 5.5 3.5 5.5Z" stroke="currentColor" strokeWidth="1.5"/>
      </svg>
      {copied ? 'Copied!' : 'Copy'}
    </button>
  )
}
