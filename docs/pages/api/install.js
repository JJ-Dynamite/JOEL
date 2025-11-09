import fs from 'fs'
import path from 'path'

export default function handler(req, res) {
  // Only allow GET requests
  if (req.method !== 'GET') {
    return res.status(405).end()
  }

  // Read the install script
  const filePath = path.join(process.cwd(), 'public', 'install')
  
  try {
    const fileContents = fs.readFileSync(filePath, 'utf8')
    
    // Set headers for bash script
    res.setHeader('Content-Type', 'text/plain; charset=utf-8')
    res.setHeader('Content-Disposition', 'inline; filename="install"')
    
    // Send the script
    res.status(200).send(fileContents)
  } catch (error) {
    console.error('Error reading install script:', error)
    res.status(500).send('Error loading install script')
  }
}

