# Building an In-App Wallet on the Internet Computer

This guide walks you through creating a full-featured in-app wallet that supports both ICP and custom ICRC-2 tokens.

## Introduction
The Internet Computer supports various wallet types, from basic to advanced implementations. Learn more about wallets on ICP here:
- [ICP Wallet Overview](https://internetcomputer.org/docs/current/developer-docs/defi/wallets/overview)

## Step 1: Create Your Custom ICRC-2 Token
You have two options for creating your custom token:

### Option A: Manual Token Creation
1. Follow the [official ICP token creation guide](https://internetcomputer.org/docs/current/developer-docs/defi/tokens/create)
2. Reference this [example ICRC-2 token implementation](https://github.com/Stephen-Kimoi/example-icrc2-token)

### Option B: No-Code Token Creation
Use [ICPex Token Creator](https://icpex.org/createToken) to create your token without writing code.

## Step 2: Integrate ICP Token Support

### Installation
```bash
npm i @dfinity/ledger-icp
npm i @dfinity/agent @dfinity/candid @dfinity/principal @dfinity/utils
``` 

### Basic Integration
1. Initialize the ICP ledger:
```typescript
import { LedgerCanister } from "@dfinity/ledger-icp";

const ledgerCanister = LedgerCanister.create({
  agent,
  canisterId: LEDGER_CANISTER_ID,
});
``` 

2. Fetch balances:
```typescript
const balance = await ledgerCanister.accountBalance({
  accountIdentifier,
  certified: true,
});
```

3. Transfer tokens:
```typescript
const blockHeight = await ledgerCanister.icrc1Transfer({
  to: recipientAccount,
  amount: transferAmount,
});
``` 

## Step 3: Integrate Your Custom ICRC Token
### Installation

```bash
npm i @dfinity/ledger-icrc
``` 


### Basic Integration

1. Initialize the ICRC ledger:
```typescript
import { IcrcLedgerCanister } from "@dfinity/ledger-icrc";

const icrcLedger = IcrcLedgerCanister.create({
  agent,
  canisterId: YOUR_TOKEN_CANISTER_ID,
});
``` 

2. Fetch token balances:
```typescript
const balance = await icrcLedger.balance({
  owner: principal,
});
``` 

3. Transfer tokens:
```typescript
const blockHeight = await icrcLedger.transfer({
  to: {
    owner: recipientPrincipal,
    subaccount: [],
  },
  amount: transferAmount,
});
``` 

## Project Structure: 

```bash 
src/
├── in_app_wallet_frontend/
│   ├── src/
│   │   ├── App.jsx        # Main wallet interface
│   │   ├── auth.js        # Authentication & token operations
│   │   └── App.css        # Styling
│   └── ...
└── ...
``` 

## Key features: 
- Internet Identity authentication
- ICP token integration
- Custom ICRC-2 token support
- Token balance display
- Token transfer functionality
- Transaction status tracking

## Development Setup: 
1. Clone the repository
2. Install dependencies:
```bash
npm install
```

## Resources: 
