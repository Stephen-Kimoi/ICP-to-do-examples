# Distributed Key Management Service: 
VetKeys can be used to faciliate a distributed key management service (DKMS)

Since vetKeys are only known to the client that requested them: the client can use it for encryption, signing and further key derivation - derived keys can be shared with other users, by adding corresponding user permissions to the canister. 

## ``KeyManager`` Library: 
Offers Access Control and VetKey derivation 

- Gives simple frontend APIs for key derivation, with no need of managing your own cryptographic primitives 
- Offers deterministic key generation. 
- Has a built in access control for keys. No peer-to-peer interaction required 
- Enhances security when fetching keys on demand. No need of persisting sesitive keys locally. 

## How does it work: 
A canister that has enabled ``KeyManager`` determines access control for vetKeys

