Cheat sheet on how to navigate certain things on ICP

## Content 
1. [ICP Tokens to cycles conversion](#icp-tokens-to-cycles-conversion) 
2. [Canister Upgrade on rust while using stable structures](#canister-upgrade-on-rust-while-using-stable-structures)
3. [Generating .did files in rust](#generating-did-files-in-rust)
<!-- 2. Updating Content Security Policy (CSP) -->
<!-- 3. Things to note about pre-upgrade and post-upgrade hooks, how to utilize them with stable memory  -->

## ICP Tokens to cycles conversion: 
Commands for converting ICP tokens to cycles.

Step 1: Get your identity principal ID: 
```bash 
dfx identity get-principal  
``` 

Step 2: Send ICP tokens to your identity principal ID using your wallet or exchange 

Step 3: Check your ICP balance: 
```bash
dfx ledger --network ic balance
```

Step 4: Convert ICP tokens to cycles using the following command:
```bash
dfx cycles convert --amount 0.7 --network ic
```

Step 6: Check your cycles balance:
```bash
dfx cycles balance --network ic
```

Step 5: Create your canister on ICP mainnet with the amount of cycles 
```bash 
 dfx canister --network ic create --with-cycles 1_000_000_000_000 <canister_name> 
```

Step 6: Deploy your canister to ICP mainnet:
```bash
dfx deploy --network ic
```

### Bonus: Depositing/topping up cycles into your canister:
Step 1a: Deposit cycles into your created canister: 
```bash
dfx canister --network ic deposit-cycles 1_000_000_000_000 <canister_id>
```

## Updating Content Security Policy (CSP):

Re-deploy the canister while re-isntalling the code 
```bash
dfx deploy --network ic --mode reinstall 
```

## Canister Upgrade on rust while using stable structures:

Step 1: First of all, Ensure that you've used stable memory to store your data. Check out the full documentation on stable memory [here](https://github.com/seniorjoinu/ic-stable-memory) 

Step 2: Build your project to generate the latest Wasm file, also ensure you've generated the lastest did files. 

You can do this using one simple command: 
```bash
npx generate-did <canister_name>
``` 

Check out full example on how to use the package [here](https://www.npmjs.com/package/canister-tools)

Step 4: Upgrade your canister using the following command:
```bash
npx upgrade-canister <backend_canister_name> --backend --mainnet ## You can remove the "--mainnet" flag if it's locally
```
### Upgrading a frontend canister:
You can easily upgrade your frontend canister, ``especially when you've updated your assets.json file`` and you'd like to re-install your canister, using the command: 
```bash
upgrade-canister <frontend_canister_name> --frontend --mainnet ## You can remove the "--mainnet" flag if it's locally
```

Check out the official Internet Computer documentation on [canister upgrades](https://internetcomputer.org/docs/current/tutorials/developer-journey/level-2/2.1-storage-persistence#upgrading-canisters)

## Generating .did files in rust: 
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

#### Prerequisites: 
Ensure you've installed [candid extractor](https://github.com/dfinity/candid): 
```bash
cargo install candid-extractor
```

And added it to your ``lib.rs`` like this: 
```rust
// Required for Candid interface generation
ic_cdk::export_candid!();
```

Check out the package over [here](https://crates.io/crates/generate-did)