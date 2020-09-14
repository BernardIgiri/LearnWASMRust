#!/usr/bin/env bash
wasm-pack build --target web --out-name wasm --out-dir ./static && cp index.html static
