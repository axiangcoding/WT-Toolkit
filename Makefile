.PHONY: dev build lint format

dev:
	yarn tauri dev

build:
	yarn tauri build

lint:
	yarn lint
	cd src-tauri && cargo fmt --check

format:
	cd src-tauri && cargo fmt