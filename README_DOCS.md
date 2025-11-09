# JOEL Documentation

This directory contains the complete documentation for the JOEL programming language, built with [mdBook](https://rust-lang.github.io/mdBook/).

## Building the Documentation

### Prerequisites

Install mdBook:

```bash
cargo install mdbook
```

### Build and Serve

```bash
cd docs
mdbook serve
```

This will:
1. Build the documentation
2. Start a local server at `http://localhost:3000`
3. Automatically reload on file changes

### Build Only

```bash
cd docs
mdbook build
```

The generated HTML will be in `docs/book/`.

## Documentation Structure

```
docs/
├── book.toml          # mdBook configuration
└── src/
    ├── SUMMARY.md     # Table of contents
    ├── introduction.md
    ├── getting-started/
    │   ├── installation.md
    │   ├── quick-start.md
    │   └── first-program.md
    ├── language/
    │   ├── syntax-overview.md
    │   ├── data-types.md
    │   ├── functions.md
    │   └── ...
    ├── examples/
    ├── toolchain/
    └── ...
```

## Adding New Documentation

1. Create a new `.md` file in the appropriate directory
2. Add an entry to `src/SUMMARY.md`
3. Write your documentation in Markdown
4. Run `mdbook serve` to preview

## Deploying Documentation

### GitHub Pages

1. Build the documentation:
   ```bash
   cd docs
   mdbook build
   ```

2. Push the `book/` directory to the `gh-pages` branch

3. Enable GitHub Pages in repository settings

### Other Platforms

The generated HTML in `docs/book/` can be deployed to any static hosting service:
- Netlify
- Vercel
- Cloudflare Pages
- AWS S3 + CloudFront

## Documentation Guidelines

- Use clear, concise language
- Include code examples
- Add syntax highlighting with ` ```joel ` blocks
- Keep examples runnable
- Update documentation when features change

## Contributing

See the main [README](../README.md) for contribution guidelines.
