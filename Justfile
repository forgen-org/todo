axum: 
    cd clients/axum && cargo watch -x run
    
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
