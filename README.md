# wasmic-wormhole

![maintenance-status](https://img.shields.io/badge/maintenance-experimental-blue.svg)

WASM/WASI bindings for the Rust version of the magic-wormhole library

### NOTE

This project is currently on hold, due to lack of support for the `wasm32-wasi` target in `wasm-pack` (which in turn is waiting for `wasm32-wasi` support in `wasm-bindgen`) (See: https://github.com/rustwasm/wasm-pack/issues/654 and https://github.com/rustwasm/wasm-bindgen/issues/3421).

Support for this target is required, as multiple peer dependencies of `magic-wormhole` (such as `errno`) don't support the `wasm32-unknown-unknown` target.

So until there's progress in adding support for `wasm32-wasi` in `wasm-pack` and `wasm-bindgen`, this project will be on hold.
