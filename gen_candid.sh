#!/bin/bash

cargo build --release --target wasm32-unknown-unknown --package etcher_backend
candid-extractor target/wasm32-unknown-unknown/release/etcher_backend.wasm > src/etcher_backend/etcher_backend.did
