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
1. Initializing the ICP ledger. Check out function [here](https://github.com/Stephen-Kimoi/ICP-to-do-examples/blob/971e735594ba364c107d06590a234f919f28a954/examples/in_app_wallet/src/in_app_wallet_frontend/src/auth.js#L26)
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

2. Fetch ICP balance, check out the function [here](https://github.com/Stephen-Kimoi/ICP-to-do-examples/blob/971e735594ba364c107d06590a234f919f28a954/examples/in_app_wallet/src/in_app_wallet_frontend/src/auth.js#L47)
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

3. Transfer ICP token, check out the function [here](https://github.com/Stephen-Kimoi/ICP-to-do-examples/blob/971e735594ba364c107d06590a234f919f28a954/examples/in_app_wallet/src/in_app_wallet_frontend/src/auth.js#L68)
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
