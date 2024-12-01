import { AuthClient } from "@dfinity/auth-client";
import { createAgent } from "@dfinity/utils";
import { LedgerCanister } from "@dfinity/ledger-icp";
import { AccountIdentifier } from "@dfinity/ledger-icp";
import { Principal } from "@dfinity/principal";

const LEDGER_CANISTER_ID = "ryjl3-tyaaa-aaaaa-aaaba-cai"; 

class AuthManager {
  constructor() {
    this.authClient = null;
    this.ledgerCanister = null;
  }

  async init() {
    this.authClient = await AuthClient.create();
    if (this.authClient.isAuthenticated()) {
      await this.initLedger();
    }
    return this.authClient.isAuthenticated();
  }

  async initLedger() {
    const identity = this.authClient.getIdentity();
    const agent = await createAgent({
      identity,
      //   host: process.env.DFX_NETWORK === 'ic' ? 'https://icp-api.io' : 'http://localhost:4943',
      host: 'https://icp-api.io',
    });

    this.ledgerCanister = LedgerCanister.create({
      agent,
      canisterId: Principal.fromText(LEDGER_CANISTER_ID),
    });
  }

  async getBalance() {
    if (!this.ledgerCanister) return BigInt(0);
    
    const identity = this.getIdentity();
    const accountIdentifier = AccountIdentifier.fromPrincipal({
      principal: identity.getPrincipal(),
    });

    const balance = await this.ledgerCanister.accountBalance({
      accountIdentifier,
      certified: true,
    });

    return balance;
  }

  async login() {
    return new Promise(async (resolve) => {
      await this.authClient?.login({
        identityProvider: 'https://identity.ic0.app', 
        // identityProvider: process.env.DFX_NETWORK === 'ic' 
        //   ? 'https://identity.ic0.app'
        //   : `http://localhost:4943/?canisterId=${process.env.INTERNET_IDENTITY_CANISTER_ID}`,
        onSuccess: async () => {
          await this.initLedger();
          resolve(true);
        },
        onError: () => resolve(false)
      });
    });
  }

  async logout() {
    await this.authClient?.logout();
    this.ledgerCanister = null;
  }

  getIdentity() {
    return this.authClient?.getIdentity();
  }
}

export const authManager = new AuthManager();
