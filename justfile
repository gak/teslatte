all_tests: no_token_test token_tests audit

no_token_test:
    cargo clippy --all-targets -- -D warnings
    cargo clippy --all-targets --all-features -- -D warnings
    cargo test

# Require an access token from "cli.json". Use `just auth` to generate.
token_tests:
    cargo run -- api products
    cargo run --no-default-features --features cli -- api products

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

auth:
    cargo run -- auth --save

audit:
    cargo audit

update:
    cargo update && cargo upgrade
    cd tesla_api_coverage && cargo update && cargo upgrade
