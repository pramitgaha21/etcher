# Etcher

### Deployment Guide

#### Pre-requisites
bitcoind
candid-extractor

##### Follow the Guide to start the `bitcoind` for `regtest` [link](https://internetcomputer.org/docs/current/developer-docs/multi-chain/bitcoin/using-btc/local-development)

```bash
# Optional Step
chmod +x gen_candid.sh
./gen_candid.sh # Generates the candid file

dfx start --clean # run the localhost in a different terminal

chmod +x deploy.sh
./deploy.sh
```
