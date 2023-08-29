all_tests: no_token_test token_tests

no_token_test:
    cargo clippy --all-targets -- -D warnings
    cargo clippy --all-targets --all-features -- -D warnings
    cargo test

# These tests require an access token saved in a file called "cli.json"
token_tests:
    cargo run -- api vehicles
    cargo run -- api energy-sites
