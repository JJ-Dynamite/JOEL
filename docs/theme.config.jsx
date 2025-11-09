import { useConfig } from 'nextra-theme-docs'

const config = {
  logo: <span><strong>JOEL</strong> Documentation</span>,
  project: {
    link: 'https://github.com/JJ-Dynamite/JOEL',
  },
  chat: {
    link: 'https://github.com/JJ-Dynamite/JOEL/discussions',
  },
  docsRepositoryBase: 'https://github.com/JJ-Dynamite/JOEL/tree/main/docs',
  footer: {
    text: 'JOEL Language Documentation © 2025',
  },
  useNextSeoProps() {
    return {
      titleTemplate: '%s – JOEL'
    }
  },
  head: (
    <>
      <meta name="viewport" content="width=device-width, initial-scale=1.0" />
      <meta property="og:title" content="JOEL Programming Language" />
      <meta property="og:description" content="A polymodal programming language for systems, AI, Web3, UI, and more" />
    </>
  ),
  sidebar: {
    defaultMenuCollapseLevel: 1,
  },
  search: {
    placeholder: 'Search documentation...',
  },
  editLink: {
    text: 'Edit this page on GitHub →'
  },
  feedback: {
    content: 'Question? Give us feedback →',
    labels: 'feedback'
  },
  toc: {
    backToTop: true,
  },
}

export default config

