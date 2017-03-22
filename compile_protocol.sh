#!/bin/sh
protoc --rust_out . protocol/kinetic.proto
mv kinetic.rs  src/kinetic/proto/raw.rs
