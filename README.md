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

4. Run JS Examples locally

```sh
cd js_food
wasmedge-tensorflow-lite --dir .:. wasmedge_quickjs.wasm main.js
cd ..
```

5. Run Rust examples locally

```sh
cd rust_mobilenet
cargo build --target wasm32-wasi --release
```

We can AOT compile our Rust code to machine native code, and then use WasmEdge sandbox to run the native code.

```sh
wasmedgec-tensorflow target/wasm32-wasi/release/classify.wasm classify.so
wasmedge-tensorflow-lite classify.so < grace_hopper.jpg
```

## Demo 2: Deploy as a FAAS

### Prerequisite:

- Install Rust with `rustup`
- Have GCC installed
- Git and cURL
- Vercel CLI

### Steps:

1. Install the `wasm32-wasi` target

```sh
rustup target add wasm32-wasi
```

2. Build the Rust program to WebAssembly bytecode

```sh
cd faas
cd api/functions/image-classification/
cargo build --release --target wasm32-wasi
```

3. Prepare the build artifacts for deployment

```sh
cp target/wasm32-wasi/release/classify.wasm ../../
```

4.  Deploy the function

```sh
cd ../../../
vercel deploy
```

## Demo 3

Build container

```sh
sudo buildah build --annotation "module.wasm.image/variant=compat" -t classify .
```