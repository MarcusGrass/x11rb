#!/bin/sh
cargo run -p x11rb-generator -- xcb-proto-1.14-1-g2b3559c/src/ x11rb/src/protocol/ x11rb/src/xcb/ && cargo fmt --all
