## Overview
This Escrow Program is a smart contract built on Solana using Anchor. It facilitates secure token swaps between users without requiring direct trust. The escrow holds tokens from a maker until a taker fulfills the exchange conditions, at which point the tokens are transferred accordingly. If the taker does not fulfill the agreement, the maker can withdraw the tokens.

## Features
* Secure token escrow using Anchor PDA (Program Derived Address).
* Supports Token 2022 and SPL Token standard.
* Allows deposit and withdrawal of tokens.
* Ensures correct token swaps by verifying mint addresses.

## Deployment Steps
1. Build & Deploy the Program:
```
anchor build
anchor deploy
```
Generate IDL:
```
anchor idl parse --file target/idl/escrow.json
```

## Conclusion
This Escrow Program provides a robust and trustless way to exchange tokens securely on Solana, leveraging Anchor's powerful framework.
