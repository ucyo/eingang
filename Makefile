both: build
	@echo "===================================================================="
	@echo "Starting backend and frontend"
	@echo "===================================================================="
	@cargo run -p eingang-backend & miniserve ./static --index index.html

build:
	@echo "===================================================================="
	@echo "Creating WebAssembly via wasm-pack"
	@echo "===================================================================="
	@wasm-pack build --target web --out-name wasm --out-dir ../static ./frontend/

frontend: build
	@echo "===================================================================="
	@echo "Serving files via miniserve"
	@echo "===================================================================="
	@miniserve ./static --index index.html

backend:
	@echo "===================================================================="
	@echo "Start backend server"
	@echo "===================================================================="
	@cargo run -p eingang-backend

clean:
	@echo "===================================================================="
	@echo "Erasing all files"
	@echo "===================================================================="
	rm -f ./static/wasm* ./static/package.json

.PHONY: both build frontend backend clean
