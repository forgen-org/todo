axium: 
    cd clients/axum && cargo watch -x run
    
yew: 
    cd clients/yew && trunk serve

deps:
    cargo +nightly udeps

lint: 
    cargo clippy
    python3 sort_derive.py
