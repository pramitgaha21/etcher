export const idlFactory = ({ IDL }) => {
        const BitcoinNetwork = IDL.Variant({
                'mainnet': IDL.Null,
                'regtest': IDL.Null,
                'testnet': IDL.Null,
        });
        const InitArgs = IDL.Record({
                'network': BitcoinNetwork,
                'ckbtc_minter': IDL.Principal,
                'schnorr_canister': IDL.Principal,
                'ckbtc_ledger': IDL.Principal,
                'timer_for_reveal_txn': IDL.Nat32,
        });
        const EtchingArgs = IDL.Record({
                'cap': IDL.Nat,
                'height': IDL.Opt(IDL.Tuple(IDL.Nat64, IDL.Nat64)),
                'turbo': IDL.Bool,
                'premine': IDL.Nat,
                'rune': IDL.Text,
                'divisibility': IDL.Nat8,
                'offset': IDL.Opt(IDL.Tuple(IDL.Nat64, IDL.Nat64)),
                'fee_rate': IDL.Opt(IDL.Nat64),
                'amount': IDL.Nat,
                'symbol': IDL.Nat32,
        });
        return IDL.Service({
                'confirm_and_convert_ckbtc': IDL.Func([], [IDL.Nat64], []),
                'etch_rune': IDL.Func([EtchingArgs], [IDL.Text, IDL.Text], []),
                'get_btc_balance': IDL.Func([], [IDL.Nat64], []),
                'get_deposit_address_for_bitcoin': IDL.Func([], [IDL.Text], []),
                'get_deposit_address_for_ckbtc': IDL.Func([], [IDL.Text], ['query']),
                'get_estimated_cbktc_conversion_fee': IDL.Func(
                        [],
                        [IDL.Nat64],
                        ['composite_query'],
                ),
                'query_conversion_status': IDL.Func(
                        [IDL.Nat64],
                        [IDL.Text],
                        ['composite_query'],
                ),
        });
};
export const init = ({ IDL }) => {
        const BitcoinNetwork = IDL.Variant({
                'mainnet': IDL.Null,
                'regtest': IDL.Null,
                'testnet': IDL.Null,
        });
        const InitArgs = IDL.Record({
                'network': BitcoinNetwork,
                'ckbtc_minter': IDL.Principal,
                'schnorr_canister': IDL.Principal,
                'ckbtc_ledger': IDL.Principal,
                'timer_for_reveal_txn': IDL.Nat32,
        });
        return [InitArgs];
};
