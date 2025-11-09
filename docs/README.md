# JOEL Documentation

This is the documentation site for the JOEL programming language, built with [Nextra](https://nextra.site/) - the same framework used for Next.js documentation.

## Getting Started

### Install Dependencies

```bash
npm install
```

### Development

Run the development server:

```bash
npm run dev
```

Open [http://localhost:3000](http://localhost:3000) in your browser.

### Build

Build the documentation for production:

```bash
npm run build
```

### Start Production Server

```bash
npm start
```

## Project Structure

```
docs/
├── pages/              # Documentation pages (MDX files)
│   ├── _meta.json     # Sidebar configuration
│   ├── getting-started/
│   ├── language/
│   ├── examples/
│   └── toolchain/
├── theme.config.jsx   # Nextra theme configuration
├── next.config.mjs    # Next.js configuration
└── package.json       # Dependencies
```

## Adding New Pages

1. Create a new `.mdx` file in the appropriate directory under `pages/`
2. Add an entry to the corresponding `_meta.json` file
3. The page will automatically appear in the sidebar

## Deployment

### Vercel (Recommended)

[![Deploy with Vercel](https://vercel.com/button)](https://vercel.com/new/clone?repository-url=https://github.com/JJ-Dynamite/JOEL)

### Other Platforms

The documentation can be deployed to any platform that supports Next.js:

- Netlify
- Cloudflare Pages
- AWS Amplify
- GitHub Pages (with static export)

## Features

- ✨ Beautiful UI matching Next.js docs
- 🔍 Built-in search functionality
- 🌙 Dark mode support
- 📱 Mobile responsive
- ⚡ Fast page loads
- 📝 Markdown/MDX support
- 🎨 Syntax highlighting

## Resources

- [Nextra Documentation](https://nextra.site/)
- [Next.js Documentation](https://nextjs.org/docs)

