# Internet Computer (ICP) Development Tutorials 

This repository contains a collection of tutorials and projects for learning various aspects of development on the [Internet Computer (ICP)](https://internetcomputer.org/) platform. 

This repository is perfect for developers looking to learn, build, and experiment with ICP.

## Table of Contents: 

### Section A: Live tutorials & examples:

1. [Uploading and Managing Assets on Canisters](./examples/asset-storage/README.md): 
    - Storing images and files on ICP

2. [Integrating Tokens in Your Project/dApp](./examples/in_app_wallet/README.md):
    - Understanding token standards
    - Adding token integration to frontends

3. [Persisiting data across canister upgrades](./examples/data_persistence/README.md)
    - Implementing data persistence across canister upgrades using rust

4. [Connecting custom domain to canister URL](./examples/canister-dns/README.md): 
    - Easily connect your custom domain to your canister URL, supporting both main domains and subdomains.

5. [Calling a canister from a Web2 API](./examples/post-likes-tutorial/README.md)
   - This tutorial demonstrates how to integrate your existing Web2 API with ICP canisters

### Section B: Cheat Sheet:
This [Cheat Sheet file](cheatsheet/README.md) contains tutorials on how to navigate certain things on ICP.

1. [Setting up your local developer environment on your PC (Mac, Linux & Windows)](cheatsheet/environment-setup.md)
2. [ICP Tokens to cycles conversion](cheatsheet/ICP-Tokens-to-cycles-conversion.md)
3. [Canister Upgrade on rust while using stable structures](cheatsheet/Canister-Upgrade-on-rust-while-using-stable-structures.md)
4. [Generating .did files in rust](cheatsheet/Generating-did-files-in-rust.md)
5. [Updating Content Security Policy](cheatsheet/updating-csp.md)
6. [Automating deployments on ICP](cheatsheet/Automating-deployments-on-ICP.md)

## Getting started:

### Step A: When running on ICP Ninja: 
All projects are compatible with ICP Ninja, just go to the respective folder and click on 

[![](https://icp.ninja/assets/open.svg)](https://icp.ninja/editor?g=https://github.com/Stephen-Kimoi/ICP-to-do-examples/)

### Step B: When running locally
1. Prerequisites:
   - Ensure you've set up your environment for ICP development. Kindly check out the [Environment Setup](cheatsheet/environment-setup.md) file for detailed instructions.

2. Clone the repository:
```bash
git clone https://github.com/Stephen-Kimoi/ICP-to-do-examples.git
cd examples
``` 

3. Project Structure:
    - Each tutorial is contained within its own directory.

    - Each directory contains a README.md file with detailed instructions.

## Contributing:
We welcome contributions! Please see our [CONTRIBUTING.md](CONTRIBUTING.md) for details.

For more information on Internet Computer development, check out:
- [IC Developer Documentation](https://internetcomputer.org/docs/current/developer-docs/)
- [DFINITY Forum](https://forum.dfinity.org/)

## License:
This project is licensed under the MIT License. See the [MIT Licence](LICENSE) file for details.