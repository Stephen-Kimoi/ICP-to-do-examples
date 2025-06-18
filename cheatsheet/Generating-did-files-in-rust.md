Cheat sheet for generating .did files in Rust canisters

## Steps

You can now easily generate .did files in rust using the command
```bash
generate-did <canister_name>
```

But first of all you need to ensure you've installed it from [crates.io](https://crates.io/crates/generate-did)
```bash
cargo install generate-did
```

This will:
1. Build the Rust canister
2. Extract and generate the Candid file
3. Save it as <canister_name>.did

### Prerequisites:
Ensure you've installed [candid extractor](https://github.com/dfinity/candid):
```bash
cargo install candid-extractor
```

And added it to your `lib.rs` like this:
```rust
// Required for Candid interface generation
ic_cdk::export_candid!();
```

Check out the package over [here](https://crates.io/crates/generate-did) 