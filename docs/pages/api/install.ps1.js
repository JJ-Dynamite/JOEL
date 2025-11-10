import { promises as fs } from 'fs'
import path from 'path'

export default async function handler(req, res) {
  try {
    const filePath = path.join(process.cwd(), 'docs', 'public', 'install.ps1')
    const scriptContent = await fs.readFile(filePath, 'utf8')

    res.setHeader('Content-Type', 'text/plain; charset=utf-8')
    res.status(200).send(scriptContent)
  } catch (error) {
    console.error('Error serving install script:', error)
    res.status(404).send('Install script not found.')
  }
}

