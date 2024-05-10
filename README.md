# Etcher

### Deployment Guide

#### Pre-requisites
- [candid-extractor](https://github.com/dfinity/cdk-rs/tree/main/src/candid-extractor)
- [dfx](https://github.com/dfinity/sdk)
- [docker](https://www.docker.com)

#### Running the docker
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

### CkBTC as fee

![CkBTC](/docs/ckbtc_as_fee.png)

### Minting CkBTC locally
1. Get the principal
```bash
dfx identity get-principal
```

2. Get the Bitcoin Deposit Address
```bash
dfx canister call ckbtc_minter get_btc_address '(record{
    owner = principal "<YOUR-PRINCIPAL>";
    subaccount = null;
})'
It will return an address
```

3. Sending Bitcoin to the address
As we're on the localhost, we will be using `bitcoin-cli` for minting some Bitcoins to the address
```bash
docker compose exec bitcoind bitcoin-cli generatetoaddress 1 <YOUR-BITCOIN-ADDRESS>

docker compose exec bitcoind bitcoin-cli -generate 101
```
After minting Bitcoins to the address, we also need to generate 100 blocks due to the coinbase maturity rule

4. Notify the `CkBTC Minter` about the deposit
```bash
dfx canister call ckbtc_minter update_balance '(record{ owner = opt principal "<YOUR-PRINCIPAL>"; subaccount = null})'
```

5. Checking the Balance
```bash
dfx canister call ckbtc_ledger icrc1_balance_of '(record{
    owner = principal "<YOUR-PRINCIPAL>";
    subaccount= null;
})'
```

### Transfer CKBTC for fee
1. Get the deposit address for CkBTC
```bash
dfx canister call etcher_backend get_deposit_address_for_ckbtc
```

2. Transfering the token
```bash
dfx canister call ckbtc_ledger icrc1_transfer '(record{
    to = record { owner = principal"<Address returned>"; };
    amount = 2_0000_0000;
    fee = opt 10;
})'
```

3. Notifying about deposit
When the canister is notified about deposit, it will also submit a transaction for conversion of CkBTC to BTC
After the call is executed successfully, it will return a block id
```bash
dfx canister call etcher_backend confirm_and_convert_ckbtc
```

4. Fetch the status of your Converstion transaction
```
dfx canister call etcher_backend query_conversion_status '(<BLOCK-ID>)'
```

5. Mint blocks to finalize the Transaction
```bash
docker compose exec bitcoind bitcoin-cli -generate 10
```

6. Checking the balance
```bash
dfx canister call etcher_backend get_btc_balance
```

### Transfer of BTC for fee
1. Get the Deposit Address for Bitcoin
```bash
dfx canister call etcher_backend get_deposit_address_for_bitcoin
```

2. Sending Bitcoins
As this tutorial is on the localhost, so we will be using `bitcoin-cli` for funding the address
```
docker compose exec bitcoind bitcoin-cli generatetoaddress 1 <Bitcoin-Address>

docker compose exec bitcoind bitcoin-cli -generate 101 # generating 101 blocks due to coinbase 100 blocks maturity rule
```

3. Checking the balance
```bash
dfx canister call etcher_backend get_btc_balance
```

### [Video Tutorial on How to Etch a Rune](https://youtu.be/Ovr51pHfNts)

### Etching Rune through Terminal

```bash
dfx canister call etcher_backend etch_rune '(record{
    rune= "DOMWOE.IS.GREAT.ARCHITECT";
    premine= 0;
    divisibility= 2;
    symbol= 65;
    cap= 20000;
    amount= 200;
    turbo= true;
    fee_rate= null;
    height= null;
    offset= opt record { 100; 200 }
})'

docker compose exec bitcoind bitcoin-cli -generate 1 # run this command on another window

docker compose exec bitcoind bitcoin-cli -generate 6 # mine 6 blocks for reveal transaction

docker compose exec bitcoind bitcoin-cli -generate 6 # run this command after the reveal transaction is submitted
```
You've successfully etched a rune, check on http:localhost:8080/runes

### Explaining the Arguments

```
type EtchingArgs = record {
  cap : nat;
  height : opt record { nat64; nat64 };
  turbo : bool;
  premine : nat;
  rune : text;
  divisibility : nat8;
  offset : opt record { nat64; nat64 };
  fee_rate : opt nat64;
  amount : nat;
  symbol : nat32;
};
```

- `rune`<br>
    Names consist of the letters A through Z and are between one and twenty-six letters long. For example UNCOMMONGOODS is a rune name. Names may contain spacers, represented as bullets, to aid readability. UNCOMMONGOODS might be etched as UNCOMMON•GOODS. The uniqueness of a name does not depend on spacers. Thus, a rune may not be etched with the same sequence of letters as an existing rune, even if it has different spacers. Spacers can only be placed between two letters. Finally, spacers do not count towards the letter count.
- `divisibility`<br>
    A rune's divisibility is how finely it may be divided into its atomic units. Divisibility is expressed as the number of digits permissible after the decimal point in an amount of runes. A rune with divisibility 0 may not be divided. A unit of a rune with divisibility 1 may be divided into ten sub-units, a rune with divisibility 2 may be divided into a hundred, and so on.
- `symbol`<br>
    A rune's currency symbol is a single Unicode code point, for example $, ⧉, or 🧿, displayed after quantities of that rune. 101 atomic units of a rune with divisibility 2 and symbol 🧿 would be rendered as 1.01 🧿. If a rune does not have a symbol, the generic currency sign ¤, also called a scarab, should be used. Here symbol as provided as number value. For e.g. `A` is 65, `😎` is 55357.
- `cap`<br>
    The number of times a rune may be minted is its cap. A mint is closed once the cap is reached.
- `premine`<br>
    The etcher of a rune may optionally allocate to themselves units of the rune being etched. This allocation is called a premine. If `premine` is equals to `cap`, it makes the runestone unmintable.
- `amount`<br>
    The amount of the token to be minted per every Mint transaction.
- `turbo`<br>
    Flag to opt in for future protocol changes. Should be a boolean value.
- `fee_rate`<br>
    The fee that will be paid per vbytes
- `height`<br>
    This field is used for setting up mint terms. For e.g. `opt record {1000; 2000}` means the runestone will be able to be minted between block of number 1000 and 2000
- `offset`<br>
        This field is used for setting up mint terms. For e.g. If `offset` was set to `opt record {1000; 2000}`, and the Etching transaction was mined at block number 1200, it means between block number `1200 + 1000` and `1200 + 2000`: The runestone is mintable

### Address for mainnet

- frontend: https://kho2y-sqaaa-aaaag-qjuta-cai.icp0.io/
- backend: https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=dyb47-nqaaa-aaaag-qjvba-cai
