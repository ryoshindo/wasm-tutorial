## adder

```shell
cargo build --target wasm32-unknown-unknown
wasm-bindgen --target no-modules --out-dir ./target/wasm32-unknown-unknown/debug/ target/wasm32-unknown-unknown/debug/adder.wasm
wasmtime --invoke add target/wasm32-unknown-unknown/debug/adder_bg.wasm 5 7
```
