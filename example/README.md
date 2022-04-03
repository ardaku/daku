# Getting Started
Instructions for building `my.wasm` from the example code:

```bash
## Debugging
cargo build --target wasm32-unknown-unknown && cp target/wasm32-unknown-unknown/debug/wasm.wasm my.wasm

## Release
RUSTFLAGS='--cfg target_os="daku" --remap-path-prefix=$PWD=_ --remap-path-prefix=$HOME/.local/lib/cargo=- --remap-path-prefix=$HOME/.local/lib/rustup=+ --remap-path-prefix=$HOME=%' cargo build --target wasm32-unknown-unknown --release && wasm-snip target/wasm32-unknown-unknown/release/wasm.wasm --snip-rust-fmt-code --snip-rust-panicking-code -o my.wasm && wasm-strip my.wasm && wasm-opt my.wasm -o my.wasm -Os
```

## Running
In order to run programs built for the wasm32-daku target, you need to use an
implementation of daku.

TODO
