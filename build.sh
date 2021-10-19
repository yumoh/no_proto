#!/bin/bash

cargo build --release

if [ -f no_proto/*.so ]; then
    rm no_proto/*so
fi

cp target/release/libno_proto.dylib no_proto/libno_proto.so