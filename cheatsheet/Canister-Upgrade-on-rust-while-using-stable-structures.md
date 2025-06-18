Cheat sheet for upgrading a Rust canister on ICP while using stable structures

## Steps

Step 1: Ensure that you've used stable memory to store your data. Check out the full documentation on stable memory [here](https://github.com/seniorjoinu/ic-stable-memory)

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
You can easily upgrade your frontend canister, especially when you've updated your assets.json file and you'd like to re-install your canister, using the command:
```bash
upgrade-canister <frontend_canister_name> --frontend --mainnet ## You can remove the "--mainnet" flag if it's locally
```

Check out the official Internet Computer documentation on [canister upgrades](https://internetcomputer.org/docs/current/tutorials/developer-journey/level-2/2.1-storage-persistence#upgrading-canisters) 