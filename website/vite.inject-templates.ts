import fs from 'node:fs'
import path from 'node:path'
import { glob } from 'glob'
import type { Plugin } from 'vite'

export const injectTemplates = (): Plugin => ({
  name: 'inject-templates',
  transformIndexHtml: async (html) => {
    const templates = (await glob('./components/*.html'))
      .map(file => {
        const
          fileName = path.basename(file, '.html'),
          contents = fs.readFileSync(file, 'utf-8')

        return `<template id="${fileName}">${contents}</template>`
      })

    return html.replace(
      '</body>',
      `${templates.join('\n')}\n</body>`,
    )
  },
})
