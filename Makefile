both: build
	@echo "===================================================================="
	@echo "Start backend and frontend"
	@echo "===================================================================="
	@cargo run -p eingang-backend & miniserve ./static --index index.html

build:
	@echo "===================================================================="
	@echo "Build WebAssembly via wasm-pack"
	@echo "===================================================================="
	@wasm-pack build --target web --out-name wasm --out-dir ../static ./frontend/

frontend: build
	@echo "===================================================================="
	@echo "Serve files via miniserve"
	@echo "===================================================================="
	@miniserve ./static --index index.html

backend:
	@echo "===================================================================="
	@echo "Start backend"
	@echo "===================================================================="
	@cargo run -p eingang-backend

clean:
	@echo "===================================================================="
	@echo "Erase all auto-generated files"
	@echo "===================================================================="
	rm -f ./static/wasm* ./static/package.json

doc:
	@echo "===================================================================="
	@echo "Build documentation and start browser"
	@echo "===================================================================="
	cargo doc --no-deps & x-www-browser target/doc/eingang/index.html

.PHONY: both build frontend backend clean doc
