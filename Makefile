serve: build
	@echo "===================================================================="
	@echo "Serving files via miniserve"
	@echo "===================================================================="
	@miniserve ./static --index index.html

build:
	@echo "===================================================================="
	@echo "Creating WebAssembly via wasm-pack"
	@echo "===================================================================="
	@wasm-pack build --target web --out-name wasm --out-dir ../static ./frontend/

clean:
	@echo "===================================================================="
	@echo "Erasing all files"
	@echo "===================================================================="
	rm -f ./static/wasm* ./static/package.json

backend:
	@echo "===================================================================="
	@echo "Start backend server"
	@echo "===================================================================="
	@cargo run -p eingang-backend

.PHONY: serve build clean backend
