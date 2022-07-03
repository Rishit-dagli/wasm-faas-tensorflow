# WebAssembly based AI as a Service with Kubernetes

## Demo 1: Run locally with WasmEdge

### Prerequisite:

- Install Rust with `rustup`
- Have GCC installed
- Git and cURL

### Steps:

1. Install the `wasm32-wasi` target

```sh
rustup target add wasm32-wasi
```

2. Install `wasmedge`

```sh
curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash
source $HOME/.wasmedge/env
```

3. Build the QuickJS interpreter with the WasmEdge Tensorflow extension

```sh
cd wasmedge-quickjs
cargo build --target wasm32-wasi --release --features=tensorflow
cd ..
```

4. Run locally

```sh
cd js_food
wasmedge-tensorflow-lite --dir .:. wasmedge_quickjs.wasm main.js
```