# Canister DNS

A tool to easily connect your custom domain to your canister URL, supporting both main domains and subdomains.

## Quick Start

Visit [canisterdns.stevekimoi.me](https://canisterdns.stevekimoi.me/) to get started.

## Domain Configuration

### 1. Generate DNS Records

1. Enter your domain and canister ID
   ![Step 2](./images/Step2.png)

2. Click "Show DNS Records" to view your DNS configuration
   ![Step 3](./images/Step3.png)

3. For www subdomain support, toggle "Show with www"
   ![step 4](./images/Step4.png)

### 2. Configure DNS Provider

1. Copy the generated DNS records
   ![DNS Records on site](./images/DNS.png)

2. Add these records to your DNS provider (e.g., Namecheap)
   ![DNS Records on namecheap](./images/DNS-namecheap.png)

### 3. Configure Canister

1. Create `.well-known/ic-domains` file in your canister's public directory
   ![ic domains file](./images/ic-domains.png)

2. Add your domain(s) to the file (one per line)
   ![ic domains file](./images/ic-domains2.png)

3. Update `.ic-assets.json5` to include the `.well-known` directory
   ![well known](./images/well-known.png)

4. Deploy your updated canister
   ![deploy](./images/deploy.png)

### 4. Register with HTTPS Gateway

Run the provided curl command to register your domain
![register](./images/register.png)

## Subdomain Configuration

### 1. Generate Subdomain DNS Records

1. Click the subdomain button
2. Enter:
   - **Domain**: Your main domain (e.g., `stevekimoi.me`)
   - **Subdomain label**: The subdomain prefix (e.g., `canisterdns`)
   - **Canister ID**: Your frontend canister ID
   ![sub domain](./images/subdomain.png)

3. Click "Show DNS Records"
   ![dns records](./images/dns2.png)

### 2. Configure DNS Provider

Add the generated records to your DNS provider
![dns records](./images/dns-namecheap2.png)

### 3. Configure Canister

1. Create `.well-known/ic-domains` file
   ![ic domains file](./images/ic-domains.png)

2. Add your subdomain to the file
   ![ic domains 3](./images/ic-domains3.png)

3. Update `.ic-assets.json5` to include `.well-known` directory
   ![well known](./images/well-known.png)

4. Deploy your updated canister
   ![deploy](./images/deploy.png)

### 4. Register with HTTPS Gateway

Run the provided curl command to register your subdomain
![curl](./images/register2.png)

--- 

Check the source code [here](https://github.com/Stephen-Kimoi/canister-dns)

Check the official documentation [here](https://internetcomputer.org/docs/building-apps/frontends/custom-domains/using-custom-domains/)