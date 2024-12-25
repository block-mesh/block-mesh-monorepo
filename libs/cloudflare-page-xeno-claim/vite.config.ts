import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import { nodePolyfills } from 'vite-plugin-node-polyfills'

// https://vite.dev/config/
export default defineConfig({
  build: {
    sourcemap: false,
    minify: true,
    cssMinify: true
  },
  server: {
    watch: {
      usePolling: true
    },
    cors: {
      allowedHeaders: '*',
      methods: '*',
      origin: '*'
    }
  },
  plugins: [react(),
    nodePolyfills()
  ]
})
