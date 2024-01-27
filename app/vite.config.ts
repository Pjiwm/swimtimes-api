import { defineConfig } from 'vite'
import ViteWindiCSSPlugin from 'vite-plugin-windicss';
import react from '@vitejs/plugin-react'


// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(), ViteWindiCSSPlugin(), ],
  build: {
    outDir: '../static'
  }
})
