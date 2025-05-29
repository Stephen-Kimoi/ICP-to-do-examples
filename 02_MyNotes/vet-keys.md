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


