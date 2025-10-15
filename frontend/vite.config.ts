import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  base: '/',  // Serve from root
  build: {
    outDir: '../dist',  // Build to backend-served dir
    emptyOutDir: true,
  },
})