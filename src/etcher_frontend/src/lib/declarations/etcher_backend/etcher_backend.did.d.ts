import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export type BitcoinNetwork = { 'mainnet': null } |
{ 'regtest': null } |
{ 'testnet': null };
export interface EtchingArgs {
        'cap': bigint,
        'height': [] | [[bigint, bigint]],
        'turbo': boolean,
        'premine': bigint,
        'rune': string,
        'divisibility': number,
        'offset': [] | [[bigint, bigint]],
        'fee_rate': [] | [bigint],
        'amount': bigint,
        'symbol': number,
}
export interface InitArgs {
        'network': BitcoinNetwork,
        'ckbtc_minter': Principal,
        'schnorr_canister': Principal,
        'ckbtc_ledger': Principal,
        'timer_for_reveal_txn': number,
}
export interface _SERVICE {
        'confirm_and_convert_ckbtc': ActorMethod<[], bigint>,
        'etch_rune': ActorMethod<[EtchingArgs], [string, string]>,
        'get_btc_balance': ActorMethod<[], bigint>,
        'get_deposit_address_for_bitcoin': ActorMethod<[], string>,
        'get_deposit_address_for_ckbtc': ActorMethod<[], string>,
        'get_estimated_cbktc_conversion_fee': ActorMethod<[], bigint>,
        'query_conversion_status': ActorMethod<[bigint], string>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: ({ IDL }: { IDL: IDL }) => IDL.Type[];
