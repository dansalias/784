import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import tailwindcss from '@tailwindcss/vite'
import { injectTemplates } from './vite.inject-templates'

export default defineConfig({
  plugins: [
    vue(),
    tailwindcss(),
    injectTemplates(),
  ],
})
