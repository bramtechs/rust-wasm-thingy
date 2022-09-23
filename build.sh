#!/bin/bash
wasm-pack build && \
cp pkg/rustwasmthingy_bg.wasm main.wasm