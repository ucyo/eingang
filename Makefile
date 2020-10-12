both: build
	@echo "===================================================================="
	@echo "Starting backend and frontend"
	@echo "===================================================================="
	@cargo run -p eingang-backend & miniserve ./static --index index.html

frontend: build
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

kill:
	@echo "===================================================================="
	@echo "Killing backend and frontend"
	@echo "===================================================================="
	@killall eingang-backend & killall miniserve


.PHONY: frontend build clean backend both kill
