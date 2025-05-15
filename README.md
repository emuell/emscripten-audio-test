# Emscripten Audio Test

Tests Emscripten audio playback via Rust.

Currently uses [sokol](https://github.com/floooh/sokol-rust.git) as audio player as [CPAL](https://github.com/RustAudio/cpal)'s emscripten support unfortunately is broken... 

## Prerequisites

```
cargo [b]install simple-http-server
rustup target add wasm32-unknown-emscripten
```

## Build

```
cargo build --target wasm32-unknown-emscripten
```

## Run

```
simple-http-server --nocache --index .
```
