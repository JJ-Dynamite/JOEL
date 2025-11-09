import nextra from 'nextra'

const withNextra = nextra({
  theme: 'nextra-theme-docs',
  themeConfig: './theme.config.jsx',
  latex: true,
  search: {
    codeblocks: false
  }
})

export default withNextra({
  reactStrictMode: true,
  basePath: process.env.NODE_ENV === 'production' ? '/JOEL' : '',
  assetPrefix: process.env.NODE_ENV === 'production' ? '/JOEL' : '',
})

