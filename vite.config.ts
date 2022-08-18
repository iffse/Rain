import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import { resolve } from 'path';

export default defineConfig({
	plugins: [svelte()],

	resolve: {
		alias: {
			"@": resolve(__dirname, "src"),
		},
	},

	clearScreen: false,
	server: {
		strictPort: true
	},
	envPrefix: ['VITE_', 'TAURI_'],
	build: {
		target: ['es2021', 'chrome100', 'safari13'],
		minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
		sourcemap: !!process.env.TAURI_DEBUG
	}
})
