#!/bin/bash

rm -rf wasm_files
mkdir wasm_files

cargo build --release --target wasm32-unknown-unknown --package etcher_backend
mv target/wasm32-unknown-unknown/release/etcher_backend.wasm ./wasm_files
candid-extractor wasm_files/etcher_backend.wasm > src/etcher_backend/etcher_backend.did
