import { defineConfig } from 'vite';
import { resolve } from 'path';

// https://vitejs.dev/config
export default defineConfig({
    build: {
        outDir: "dist",
        rollupOptions: {
            input: {
                main: resolve(__dirname, 'src/renderer/frontend/index.html')
            },
            output: {
                dir: 'dist/server'
            }
        }
    },
});
