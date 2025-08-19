# Introduction to VetKeys: 

VetKeys: Verifiably Encrypted Threshold Keys

Allow developers to implement: 
- Encryption 
- Threshold Decryption 
- Threshold signing 

Powered by vetKD protocol: verifiably encrypted threshold key derivation 

vetKeys allow public blockchains like ICP to hold and work with secret data securely

## What's the goal: 
Encrypting data locally and storing it onchain is straghtforward - since the secret key stays on the device 

The challange is when a user tries accessing the data from another device or tries sending it to someone - we did not have any private way of sending such data across public blockchains. 

Solution: vetKeys - which allow secure, on-demand key access without exposing secrets. 

## Why are they called vet keys:
- **Verifiable**: users can verify the correctness of the derived vet keys and that they haven't been tampered with 
- **Encrypted**: the derived vet keys are encrypted under a user provided transport key. It ensures that neither the individual nodes nor the network have access to the user's private keys. 
- **Threshold**: they are derived in a distributed manner, requires contribution of threshold nodes in a subnet. No node can see the derived vet keys. Thus no central authority can derive or construct them. 
- **Keys**: key material that can be further used to derive strong cryptographic keys, such as encryption and signature keys. 

## Distributed Key Management Service (DKMS):
vetKeys enables development of DKMS that enables users to easily generate, retrieve, and share cryptographic keys across devices and with other users.

## Encrypted onchain storage: 
Core application of DKMS is generation of encyption keys for securing data, whether stored in a canister, on another blockchain, or off-chain entirely. 

## Threshold BLS Signatures: 
vetKeys introduce a new threshold signature scheme to canisters - threshold BLS signatures. 

BLS signatures are particularly well-suited for multichain applications due to their compact size and efficient aggregation properties

## Identity Based Encryption (IBE):
vetKeys enables IBE, allowing data to be encrypted directly to an identity, such as a principal, an Internet Identity, an email address, or even an Ethereum address. 

Thus possible to encrypt data of a specific user or account, even if the user has not previously interacted with the dApp. 

## Timelock encryption: 
A special variant of IBE is called timelock encryption. It allows a sender to encrypt to a specific timestamp, ensuring that the recipient can only decrypt after the specific time has passed.

Canisters can enforce this time-based access control by requesting threshold decryption of a ciphertext only after a predetermined expiry time, keeping sensitive information secret until the appropriate moment. 

Timelock encryption also serves as a key building block for protecting against maximal extractable value (MEV). 

## Externally verifiable randomness: 
vetKeys can also function as a Verifiable Random Function (VRF).

[Source](https://internetcomputer.org/docs/building-apps/network-features/vetkeys/introduction)