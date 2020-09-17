serve: build
	@echo "===================================================================="
	@echo "Serving files via miniserve"
	@echo "===================================================================="
	@miniserve ./frontend/static --index index.html

build:
	@echo "===================================================================="
	@echo "Creating WebAssembly via wasm-pack"
	@echo "===================================================================="
	@wasm-pack build --target web --out-name wasm --out-dir static ./frontend/

clean:
	@echo "===================================================================="
	@echo "Erasing all files"
	@echo "===================================================================="
	rm -f ./frontend/static/wasm* ./frontend/static/package.json
