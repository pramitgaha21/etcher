# Etcher

# TODO
[ ] Payment using CkBTC
[ ] Restricting Etching of runestone based on block number

### Deployment Guide

#### Pre-requisites
bitcoind
candid-extractor

Running the docker
```bash
# for linux users
./init.sh

# for mac users
DOCKER_DEFAULT_PLATFORM=linux/amd64 ./init.sh
```

```bash
# Optional Step
chmod +x gen_candid.sh
./gen_candid_and_wasm.sh # Generates the candid file

dfx start --clean # run the localhost in a different screen

chmod +x deploy.sh
./deploy.sh # Deploys all the canisters
```
