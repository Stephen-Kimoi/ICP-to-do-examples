Cheat sheet on how to navigate certain things on ICP

## Content 
1. [ICP Tokens to cycles conversion](https://github.com/Stephen-Kimoi/ICP-to-do-examples/blob/main/CHEATSHEAT.md#icp-tokens-to-cycles-conversion) 
2. [Canister Upgrade on rust while using stable structures](https://github.com/Stephen-Kimoi/ICP-to-do-examples/blob/main/CHEATSHEAT.md#canister-upgrade-on-rust-while-using-stable-structures)
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
```bash
dfx build <canister_name> --network ic
```

Step 2: Check the current path to your wasm file: 
```bash
find . -name "<canister_name>.wasm" 
```

You'll see something like this:
```bash
./target/wasm32-unknown-unknown/release/<canister_name>.wasm
./target/wasm32-unknown-unknown/release/deps/<canister_name>.wasm
./.dfx/playground/canisters/canister_one/<canister_name>.wasm
./.dfx/local/canisters/canister_one/<canister_name>.wasm
./.dfx/ic/canisters/canister_one/<canister_name>.wasm
```

Step 3: Check your canister ID:
```bash
dfx canister id <canister_name> --network ic  
```

Step 4: Upgrade your canister using the following command:
```bash
dfx canister --network ic install <canister_id> --mode upgrade --wasm .dfx/ic/canisters/<canister_name>/<canister_name>.wasm
```
### Upgrading a frontend canister:
Step 1: Ensure you've built your frontend canister:
```bash
dfx build frontend_canister_name --network ic  
```
Step 2: Function for upgrading the frontend canister: 
```bash 
dfx canister install frontend_canister_name --mode upgrade --network ic
```

Check out the official Internet Computer documentation on [canister upgrades](https://internetcomputer.org/docs/current/tutorials/developer-journey/level-2/2.1-storage-persistence#upgrading-canisters)