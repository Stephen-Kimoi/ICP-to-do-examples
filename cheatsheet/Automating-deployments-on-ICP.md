# Automating Deployments on ICP

This guide explains how to automate deployments for Internet Computer (ICP) projects using the [Internet Computer Deploy Action](https://github.com/Stephen-Kimoi/ic-deploy-action) GitHub Action.

---

## Usage & Prerequisites

- **Mainnet Deployment:** Ensure you have deployed your canisters first. This action relies on the `canister_ids.json` file that is generated during deployment.
- **GitHub Secrets:** Set up your `IC_PEM_KEY` secret in your GitHub repository settings. See the PEM key generation section below for instructions.

### Recommended Project Structure

```
your-project/
├── src/
│   └── backend/                  # Backend Rust canister source
│   └── frontend/                 # Frontend source (if any)
│       └── dist/                 # Built frontend assets
├── dfx.json                      # DFINITY project configuration
├── canister_ids.json             # Canister IDs (generated after deployment)
├── Cargo.toml                    # Rust package manifest (for backend)
├── package.json                  # Node.js manifest (for frontend, if any)
├── tsconfig.json                 # TypeScript config (if any)
├── node_modules/                 # Node.js dependencies (if any)
└── ...
```
- Adjust `backend` and `frontend` to match your actual canister/package names.
- The action expects the backend and frontend directories to be specified in the workflow inputs.

---

## Setup Steps

1. **Deploy Canisters Locally:**
   ```bash
   dfx deploy --network ic
   ```
   This generates the required `canister_ids.json` file.

2. **Create Workflow Directory:**
   ```bash
   mkdir -p .github/workflows
   ```

3. **Create Deploy File:**
   Create a `deploy.yml` file inside the `.github/workflows` directory.

4. **Add Workflow Content:**
   Paste the following into your `deploy.yml` file:

   ```yaml
   name: Deploy to IC

   on:
     push:
       branches: [ main ]
     workflow_dispatch:

   jobs:
     deploy:
       runs-on: ubuntu-latest
       steps:
         - uses: actions/checkout@v3
         - name: Deploy to IC
           uses: Stephen-Kimoi/ic-deploy-action/action@v0.1.0
           with:
             pem_key: ${{ secrets.IC_PEM_KEY }}
             network: 'ic'
             canister_name: 'your_backend_canister_name'
             deploy_frontend: 'true'
             frontend_dir: 'src/your_frontend_dir/dist'
             backend_package: 'your_backend_package'
             frontend_package: 'your_frontend_package'
             frontend_src_dir: 'src/your_frontend_dir'
   ```

5. **Generate PEM Key:**
   - Create a new identity:
     ```bash
     dfx identity new test-identity --storage-mode=plaintext
     ```
   - Export the identity to a PEM file:
     ```bash
     dfx identity export test-identity > test-identity.pem
     ```
   - Convert to base64:
     ```bash
     base64 -i test-identity.pem > test-identity.pem.base64
     ```

6. **Add to GitHub Secrets:**
   - Add your base64-encoded PEM key as `IC_PEM_KEY` in your GitHub repository secrets.
   - Go to your repository settings, scroll to "Secrets and Variables" > "Actions", and add a new repository secret named `IC_PEM_KEY`.

7. **Push to GitHub:**
   - Push your code to GitHub and the deployment will run automatically on push to `main` or via manual workflow dispatch.

---

For more details and advanced usage, see the [ic-deploy-action README](https://github.com/Stephen-Kimoi/ic-deploy-action). 