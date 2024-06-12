.PHONY: dev build lint format

dev:
	yarn tauri dev

build:
	yarn tauri build

lint:
	yarn tsc
	yarn lint
	cd src-tauri && cargo fmt --check

format:
	yarn format
	cd src-tauri && cargo fmt