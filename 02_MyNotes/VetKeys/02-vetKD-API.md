# vetKD API
Distrubuted protocol for securely deriving cryptographic keys, referred to as **VetKeys**

Canisters can request VetKeys using this protocol (API)

Key derivation is deterministic - same inputs will produce the same keys. Thus canisters can retrieve keys reliably. 

### Methods of the API: 
1. ``vetkd_derive_key``: derive keys based on provided inputs 
2. ``vetkd_public_key``: a public key that can be used to verify keys derived using ``vetkd_derive_key``

Input parameters shared by both methods:
- ``context (blob)``: ensures keys are derived within specific context - thus preventing collisions. 
- ``key_id (record { curve: vetkd_curve; name : text })``: master key from which derived key is created. 
- 
