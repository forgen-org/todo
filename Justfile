axum: 
    cd clients/axum && cargo watch -x run

ios:
    PROJECT_DIR="${PWD}/clients/ios/TodoList" ./clients/ios/TodoList/build-rust.sh
    
yew: 
    cd clients/yew && trunk serve

test:
    cargo test -- --nocapture

deps:
    cargo +nightly udeps

lint: 
    cargo check
    cargo clippy
    python3 sort_derive.py
