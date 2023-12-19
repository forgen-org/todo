dev-yew: 
    cd clients/yew && trunk serve

deps:
    cargo +nightly udeps

lint: 
    cargo clippy
