# Etcher

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
