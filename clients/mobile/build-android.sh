#!/bin/bash

# Set up cargo-ndk
cargo install cargo-ndk

# Add the android targets
rustup target add \
        aarch64-linux-android \
        armv7-linux-androideabi
        i686-linux-android \
        x86_64-linux-android \

# Build the dylib
cargo build
        
# Build the android libraries in the jniLibs directory
cargo ndk -o android/app/src/main/jniLibs \
        --manifest-path ./Cargo.toml \
        -t armeabi-v7a \
        -t arm64-v8a \
        -t x86 \
        -t x86_64 \
        build --release 

# Create bindings
cargo run --bin uniffi-bindgen generate --library ../../target/debug/libmobile.dylib --language kotlin --out-dir android/app/src/main/java/tech/forgen/todolist/rust
