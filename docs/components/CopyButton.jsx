'use client'

export default function CopyButton({ text }) {
  const handleCopy = () => {
    if (typeof navigator !== 'undefined' && navigator.clipboard) {
      navigator.clipboard.writeText(text)
      alert('Copied to clipboard!')
    }
  }

  return (
    <button className="copy-btn" onClick={handleCopy}>
      Copy
    </button>
  )
}

