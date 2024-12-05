import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig(({ mode }) => ({
	plugins: [react()],
	server: {
		port: 3000,
	},
	build: {
		outDir: 'dist',
		emptyOutDir: true,
		sourcemap: true,
	},
}))
