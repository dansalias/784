import { defineConfig } from 'vite'
import tailwindcss from '@tailwindcss/vite'
import { injectTemplates } from './vite.inject-templates'

export default defineConfig({
  plugins: [
    tailwindcss(),
    injectTemplates(),
  ],
})
