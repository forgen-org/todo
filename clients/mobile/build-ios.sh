#!/bin/bash

# Build the dylib file
cargo build

# Create bindings
cargo run --bin uniffi-bindgen generate --library ../../target/debug/libmobile.dylib --language swift --out-dir ./bindings

# Add the ios targets and build
for TARGET in \
        aarch64-apple-ios \
        aarch64-apple-ios-sim
do
    rustup target add $TARGET
    # Apple's App Sandbox disallows SysV semaphores; use POSIX semaphores instead
    cargo build --release --target=$TARGET
done

# Rename *.modulemap to module.modulemap
mv ./bindings/mobileFFI.modulemap ./bindings/module.modulemap

# Move the Swift file to the project
rm ./ios/TodoList/Mobile.swift
mv ./bindings/mobile.swift ./ios/TodoList/Mobile.swift

# # Recreate XCFramework
rm -rf "ios/Mobile.xcframework"
xcodebuild -create-xcframework \
        -library ../../target/aarch64-apple-ios-sim/release/libmobile.a -headers ./bindings \
        -library ../../target/aarch64-apple-ios/release/libmobile.a -headers ./bindings \
        -output "ios/Mobile.xcframework"

# # Cleanup
rm -rf bindings
