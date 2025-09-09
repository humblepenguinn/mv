.PHONY: help dev-desktop dev-web build-wasm clean install-deps

help:
	@echo "Available commands:"
	@echo "  install-web-deps 	- Install required dependencies before running the web app"
	@echo "  dev-desktop  		- Run desktop app"
	@echo "  dev-web      		- Run web app"
	@echo "  build-wasm   		- Build WASM package and copy to the public folder"
	@echo "  clean        		- Clean build artifacts"

install-web-deps:
	rustup target add wasm32-unknown-unknown
	cargo install wasm-pack

dev-desktop:
	pnpm run dev:tauri

dev-web: build-wasm
	pnpm run dev

build-wasm:
	cd src-wasm && wasm-pack build --target web --out-dir pkg

clean: 
	rm -rf target/
	rm -rf src-wasm/pkg

	rm -rf plugins/vite-plugin-wasm-pack/node_modules plugins/vite-plugin-wasm-pack/dist

	rm -rf node_modules dist .vite .tanstack