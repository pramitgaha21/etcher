# Etcher

# TODO
- Payment using CkBTC

### Deployment Guide

#### Pre-requisites
[candid-extractor](https://github.com/dfinity/cdk-rs/tree/main/src/candid-extractor)
[dfx](https://github.com/dfinity/sdk)
[docker](https://www.docker.com)

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

### Ordinal Server
You can access the Ordinal server at http://localhost:8080

### Known Issues
- After the timer hits for Rune's reveal transaction while requesting the `Utxos`, The Canister might panic due to Bitcoin's canister not being fully synced. This will cause the reveal transaction to be submitted on the next timer interval

### Architecture

![Architecture](/docs/architecture.png)

### [Video Tutorial on How to Etch a Rune](https://www.youtube.com/watch?v=EbCmAyiYuJo)

### Etching Rune through Terminal

```bash
dfx canister call etcher_backend get_deposit_address_for_bitcoin # returns a bitcoin address

docker compose exec bitcoind bitcoin-cli generatetoaddress 1 <Bitcoin-Address>

docker compose exec bitcoind bitcoin-cli -generate 101 # generating 101 blocks due to coinbase 100 blocks maturity rule

dfx canister call etcher_backend etch_rune '(record{
    rune= "DOMWOE.IS.GREAT.ARCHITECT";
    divisibility= 2;
    cap= 10000;
    symbol= 65;
    premine= null;
    fee_rate= null;
    amount= 200;
    turbo= true;
    height_start= 200;
    height_stop= 1000;
    offset_start= 300;
    offset_stop= 1700;
})'

docker compose exec bitcoind bitcoin-cli -generate 1 # run this command on another window

docker compose exec bitcoind bitcoin-cli -generate 6 # mine 6 blocks for reveal transaction

docker compose exec bitcoind bitcoin-cli -generate 6 # run this command after the reveal transaction is submitted
```
You've successfully etched a rune, check on http:localhost:8080/runes

### Address for mainnet

frontend: https://kho2y-sqaaa-aaaag-qjuta-cai.icp0.io/
backend: 
