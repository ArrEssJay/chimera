import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export default defineConfig({
  plugins: [react()],
  root: '.',
  publicDir: 'public',
  base: '/',
  build: {
    outDir: 'dist-react',
    emptyOutDir: true,
    minify: 'terser',
    sourcemap: false,
    terserOptions: {
      compress: {
        drop_console: true,
        drop_debugger: true,
      },
    },
    rollupOptions: {
      output: {
        manualChunks: {
          // Vendor chunk for React and React-DOM
          'react-vendor': ['react', 'react-dom'],
          // Separate chunk for state management
          'state-management': ['zustand'],
          // Separate chunk for charting library
          'charts': ['recharts'],
        },
        // Name chunks for better caching
        chunkFileNames: 'assets/[name]-[hash].js',
        entryFileNames: 'assets/[name]-[hash].js',
        assetFileNames: 'assets/[name]-[hash].[ext]',
      },
    },
    // Target modern browsers for smaller bundles
    target: 'esnext',
    // Chunk size warnings
    chunkSizeWarningLimit: 1000,
    // Enable CSS code splitting
    cssCodeSplit: true,
  },
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src-react'),
    },
  },
  // Production optimizations
  esbuild: {
    legalComments: 'none',
  },
});
