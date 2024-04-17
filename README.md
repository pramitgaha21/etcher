# Etcher

### Deployment Guide

#### Pre-requisites
bitcoind
candid-extractor

##### Follow the Guide to start the `bitcoind` for `regtest` [link](https://internetcomputer.org/docs/current/developer-docs/multi-chain/bitcoin/using-btc/local-development)

```bash
# Optional Step
chmod +x gen_candid.sh
./gen_candid.sh

dfx start --clean # run the localhost in a different terminal

dfx deploy --specified-id ml52i-qqaaa-aaaar-qaaba-cai ckbtc_minter --argument '(variant {
    Init = record{
        btc_network = variant { regtest };
        ledger_id = principal "mc6ru-gyaaa-aaaar-qaaaq-cai";
        ecdsa_key_name = "dfx_test_key";
        retrieve_btc_min_amount = 10_000;
        max_time_in_queue_nanos = 420_000_000_000;
        min_confirmations = opt 12;
        mode = variant { GeneralAvailability };
        kyt_fee = opt 1_333;
        kyt_principal = opt principal "pvm5g-xaaaa-aaaar-qaaia-cai";
    }
})'
```
