all_tests: no_token_test token_tests

no_token_test:
    cargo clippy --all-targets -- -D warnings
    cargo clippy --all-targets --all-features -- -D warnings
    cargo test

# These tests require an access token saved in a file called "cli.json"
token_tests:
    cargo run -- api vehicles
    cargo run --no-default-features --features cli -- api vehicles
    cargo run -- api products

publish version:
    git diff-index --quiet HEAD

    # BSD sed
    sed -i '' "s/^version = \".*\"$/version = \"{{version}}\"/" Cargo.toml

    git add Cargo.toml
    git commit -m "chore: v{{version}}"
    git tag "v{{version}}"
    git push origin "v{{version}}"
    git push

    cargo publish
