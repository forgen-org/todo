dev-axum: 
    cd clients/axum && cargo watch -x run
    
dev-yew: 
    cd clients/yew && trunk serve

deps:
    cargo +nightly udeps

lint: 
    cargo clippy
