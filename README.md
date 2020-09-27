# Eingang

Minimal note taking application via WASM & Rust.

# Stages

- [x] Interactive session example
- [x] Temporary storage of data (across reloads)
- [x] Temporary storage of data (across sessions)
- [x] Use common crate for models between backend and frontend
- [x] Enable persistent storage of data (on disk) for Backend API
- [ ] Use safe/load from persistent storage via Backend API

# Todo

- Implement similar structure like `todo` example [here](https://github.com/actix/examples/blob/master/sqlx_todo/src/todo/model.rs).

# Usage

First, generate the WASM bindings and save them in `./static/`.
Afterwards, serve the files using any web server e.g. [`miniserve`](https://lib.rs/crates/miniserve).

1. Build WASM bindings
```bash
make build  # or wasm-pack build --target web --out-name wasm --out-dir ../static ./frontend/
```

2. Serve files
```bash
make serve  # or miniserve ./static --index index.html
```

The default `make` command is `make serve`. Erase all created files via following command:
```bash
make clean  # or rm -f ./static/wasm* ./static/package.json
```
