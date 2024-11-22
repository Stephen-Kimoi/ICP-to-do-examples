Cheat sheet on how to navigate certain things on ICP

## Content 
1. [ICP Tokens to cycles conversion](https://github.com/Stephen-Kimoi/ICP-to-do-examples/blob/main/CHEATSHEAT.md#icp-tokens-to-cycles-conversion) 
2. Updating Content Security Policy (CSP)

### ICP Tokens to cycles conversion: 
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