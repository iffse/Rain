.DEFAULT_GOAL := run

install:
	npm install

run: install
	npx tauri dev

build: install
	npx tauri build
