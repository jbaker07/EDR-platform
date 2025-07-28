import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import tsconfigPaths from 'vite-tsconfig-paths'
import path from 'path'

export default defineConfig({
  plugins: [
    react(),
    tsconfigPaths(),               // picks up "@/…" → "src/…" from tsconfig.json
  ],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, 'src'),
    },
  },
  server: {
    proxy: {
      '/api': {
        target: 'http://localhost:8000',
        changeOrigin: true,
        secure: false,
      },
    },
  },
  optimizeDeps: {
    // don’t pre-bundle these native/node-only modules
    exclude: ['fsevents', 'lightningcss']
  },
  build: {
    rollupOptions: {
      // treat them as external so UI bundle skips over them
      external: ['fsevents', 'lightningcss']
    }
  }
})
