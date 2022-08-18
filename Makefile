install:
	npm install
	npm install --save-dev @tauri-apps/cli

run: install
	npx tauri dev
