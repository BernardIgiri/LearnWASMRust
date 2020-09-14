#!/usr/bin/env bash
cd lib && \
  cargo build && \
  wasm-pack build && \
  cd ../ && \
  npm run build --prefix web
