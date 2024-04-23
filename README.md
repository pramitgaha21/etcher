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
./deploy.sh # Deploys all the canisters
```

dfx canister call etcher_backend etch_rune '(record{
cap= null;
offset_stop= null;
height_stop=null;
height_start=null;
offset_start=null;
turbo=true;
divisibility=2;
rune="AA";
amount=null;
symbol=null;
})'
