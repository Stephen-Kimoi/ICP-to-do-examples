# Building an In-App Wallet on the Internet Computer

This guide walks you through creating a full-featured in-app wallet that supports both ICP and custom ICRC-2 tokens.

## Introduction
The Internet Computer supports various wallet types, from basic to advanced implementations. Learn more about wallets on ICP here:
- [ICP Wallet Overview](https://internetcomputer.org/docs/current/developer-docs/defi/wallets/overview)

## Step 1(a): Create Your Custom ICRC-2 Token
You have two options for creating your custom token:

### Option A: Manual Token Creation
1. Follow the [official ICP token creation guide](https://internetcomputer.org/docs/current/developer-docs/defi/tokens/create)
2. Reference this [example ICRC-2 token implementation](https://github.com/Stephen-Kimoi/example-icrc2-token)

### Option B: No-Code Token Creation
Use [ICPex Token Creator](https://icpex.org/createToken) to create your token without writing code.

## Step 1(b): Run the project on ICP Ninja: 

[![](https://icp.ninja/assets/open.svg)](https://icp.ninja/i?g=https://github.com/dfinity/examples/rust/llm_chatbot)

### Basic Integration
1. Initializing the ICP ledger. Check out function [here](../in_app_wallet/frontend/src/auth.js#L26)
```typescript
// Import the ledger canister
import { LedgerCanister } from "@dfinity/ledger-icp";
import { AuthClient } from "@dfinity/auth-client"; 
import { Principal } from "@dfinity/principal";

// ICP Ledger Canister ID
const LEDGER_CANISTER_ID = "ryjl3-tyaaa-aaaaa-aaaba-cai"; 

// Initialize the ledger canister
async init() {
  this.authClient = await AuthClient.create();
  if (this.authClient.isAuthenticated()) {
      await this.initLedgers();
  }
  return this.authClient.isAuthenticated();
}

async initLedgers() {
    const identity = this.authClient.getIdentity();
    const agent = await createAgent({
        identity,
        // host: process.env.DFX_NETWORK === 'ic' ? 'https://icp-api.io' : 'http://localhost:4943',
        host: 'https://icp-api.io'
    });

    // Initialize ICP ledger
    this.ledgerCanister = LedgerCanister.create({
        agent,
        canisterId: Principal.fromText(LEDGER_CANISTER_ID),
    });

    // Rest of code... 
}
``` 

2. Fetch ICP balance, check out the function [here](../in_app_wallet/frontend/src/auth.js#L47)
```typescript
async getBalances() {
  const identity = this.getIdentity();
  const principal = identity.getPrincipal();

  const icpBalance = await this.ledgerCanister?.accountBalance({
      accountIdentifier: AccountIdentifier.fromPrincipal({
      principal: principal,
      }),
      certified: true,
  }) || BigInt(0);

  // Console log the ICP balance

  //  Rest of code... 
}
```

3. Transfer ICP token, check out the function [here](../in_app_wallet/frontend/src/auth.js#L68)
```typescript
  async transferICP(toPrincipal, amount) {
    if (!this.ledgerCanister) return null;
    
    const toAccount = {
      owner: Principal.fromText(toPrincipal),
      subaccount: [], // default subaccount
    };

    const transferAmount = BigInt(Math.floor(amount * 100000000)); // Convert to e8s

    const request = {
      to: toAccount,
      amount: transferAmount,
      createdAt: BigInt(Date.now() * 1000000), // Convert to nanoseconds
    };

    try {
      const blockHeight = await this.ledgerCanister.icrc1Transfer(request);
      return blockHeight;
    } catch (error) {
      console.error('Transfer failed:', error);
      throw error;
    }
  }
``` 

## Step 3: Integrating Your Custom ICRC Token
### Installation

```bash
npm i @dfinity/ledger-icrc 
npm i @dfinity/agent @dfinity/candid @dfinity/principal @dfinity/utils
``` 

### Basic Integration

1. Initialize the ICRC ledger. Check out the function [here](../in_app_wallet/frontend/src/auth.js#L41)
```typescript
import { IcrcLedgerCanister } from "@dfinity/ledger-icrc";
import { Principal } from "@dfinity/principal";

// Your custom ICRC-2 token canister ID
const INWT_CANISTER_ID = "lradw-laaaa-aaaam-acrda-cai"; 

async initLedgers() {
  const identity = this.authClient.getIdentity();
  // Previous code... 

  // Initialize INWT ledger
  this.icrcLedgerCanister = IcrcLedgerCanister.create({
      agent,
      canisterId: Principal.fromText(INWT_CANISTER_ID),
  });
}
``` 

2. Fetch custom token balances. Check out the function [here](../in_app_wallet/frontend/src/auth.js#L58)
```typescript
async getBalances() {
  const identity = this.getIdentity();
  const principal = identity.getPrincipal();
  
  // Previous code...

  const inwtBalance = await this.icrcLedgerCanister?.balance({
      owner: principal,
  }) || BigInt(0);

  // Console log the custom token balance
}
``` 

3. Transfer custom token from one account to another. Check out the function [here](../in_app_wallet/frontend/src/auth.js#L93)
```typescript
async transferINWT(toPrincipal, amount) {
  if (!this.icrcLedgerCanister) return null;

  const transferAmount = BigInt(Math.floor(amount * 100000000)); // Assuming 8 decimals

  const request = {
    to: {
      owner: Principal.fromText(toPrincipal),
      subaccount: [],
    },
    amount: transferAmount,
    created_at_time: BigInt(Date.now() * 1000000),
  };

  try {
    const blockHeight = await this.icrcLedgerCanister.transfer(request);
    return blockHeight;
  } catch (error) {
    console.error('INWT Transfer failed:', error);
    throw error;
  }
}
``` 

## Project Structure: 

```bash 
examples/in_app_wallet/
├── frontend/
│   ├── src/
│   │   ├── main.jsx       # Main wallet interface
│   │   ├── auth.js        # Authentication & token operations
│   │   └── index.css      # Styling
│   ├── public/            # Static assets
│   └── package.json       # Frontend dependencies
├── backend/
│   ├── src/
│   │   └── lib.rs         # Backend canister code
│   └── Cargo.toml         # Rust dependencies
└── dfx.json               # DFX configuration
``` 

## Key features: 
- Internet Identity authentication
- ICP token integration
- Custom ICRC-2 token support
- Token balance display
- Token transfer functionality
- Transaction status tracking