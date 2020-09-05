# Eingang

Minimal note taking application via WASM & Rust.

# Usage

First, generate the WASM bindings and save them in `./static/`.
Afterwards, serve the files using any web server e.g. [`miniserve`](https://lib.rs/crates/miniserve).

1. Build WASM bindings
```bash
make build  # or wasm-pack build --target web --out-name wasm --out-dir ./static
```

2. Serve files
```bash
make serve  # or miniserve ./static --index index.html
```
The default `make` command is `make serve`. Erase all created files via following command:
```
make clean  # or rm -f ./static/wasm* ./static/package.json
```
