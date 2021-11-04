#!/bin/bash

cargo build --release

if [ -f no_proto/*.so ]; then
    rm no_proto/*so
fi

if [ $(uname) == "Darwin" ]; then
    cp target/release/libno_proto.so no_proto/libno_proto.so
else
    cp target/release/libno_proto.dylib no_proto/libno_proto.so
fi

pip wheel .
mv *.whl target/