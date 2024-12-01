import { AuthClient } from "@dfinity/auth-client";
import { createAgent } from "@dfinity/utils";
import { LedgerCanister } from "@dfinity/ledger-icp";
import { IcrcLedgerCanister } from "@dfinity/ledger-icrc";
import { AccountIdentifier } from "@dfinity/ledger-icp";
import { Principal } from "@dfinity/principal";

const LEDGER_CANISTER_ID = "ryjl3-tyaaa-aaaaa-aaaba-cai"; 
const INWT_CANISTER_ID = "lradw-laaaa-aaaam-acrda-cai"; 

class AuthManager {
    constructor() {
        this.authClient = null;
        this.ledgerCanister = null;
        this.icrcLedgerCanister = null;
    }

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

        // Initialize INWT ledger
        this.icrcLedgerCanister = IcrcLedgerCanister.create({
            agent,
            canisterId: Principal.fromText(INWT_CANISTER_ID),
        });
    }

    async getBalances() {
        const identity = this.getIdentity();
        const principal = identity.getPrincipal();

        const icpBalance = await this.ledgerCanister?.accountBalance({
            accountIdentifier: AccountIdentifier.fromPrincipal({
            principal: principal,
            }),
            certified: true,
        }) || BigInt(0);

        const inwtBalance = await this.icrcLedgerCanister?.balance({
            owner: principal,
        }) || BigInt(0);

        return {
            ICP: icpBalance,
            INWT: inwtBalance
        };
    }

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

  async login() {
    return new Promise(async (resolve) => {
      await this.authClient?.login({
        identityProvider: 'https://identity.ic0.app', 
        // identityProvider: process.env.DFX_NETWORK === 'ic' 
        //   ? 'https://identity.ic0.app'
        //   : `http://localhost:4943/?canisterId=${process.env.INTERNET_IDENTITY_CANISTER_ID}`,
        onSuccess: async () => {
          await this.initLedgers();
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
