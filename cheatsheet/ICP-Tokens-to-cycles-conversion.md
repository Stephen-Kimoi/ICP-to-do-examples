Cheat sheet for converting ICP Tokens to cycles on the Internet Computer

## Steps

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