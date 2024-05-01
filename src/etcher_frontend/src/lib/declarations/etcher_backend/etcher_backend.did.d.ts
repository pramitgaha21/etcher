import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Account {
  'owner' : Principal,
  'subaccount' : [] | [Uint8Array | number[]],
}
export type BitcoinNetwork = { 'mainnet' : null } |
  { 'regtest' : null } |
  { 'testnet' : null };
export interface EtchingArgs {
  'cap' : bigint,
  'offset_stop' : bigint,
  'height_stop' : bigint,
  'turbo' : boolean,
  'offset_start' : bigint,
  'premine' : [] | [bigint],
  'rune' : string,
  'divisibility' : number,
  'fee_rate' : [] | [bigint],
  'amount' : bigint,
  'height_start' : bigint,
  'symbol' : number,
}
export interface InitArgs {
  'network' : BitcoinNetwork,
  'ckbtc_minter' : Principal,
  'schnorr_canister' : Principal,
  'ckbtc_ledger' : Principal,
  'timer_for_reveal_txn' : number,
}
export interface ReimbursementDeposit {
  'account' : Account,
  'mint_block_index' : bigint,
  'amount' : bigint,
  'reason' : ReimbursementReason,
}
export type ReimbursementReason = { 'CallFailed' : null } |
  { 'TaintedDestination' : { 'kyt_fee' : bigint, 'kyt_provider' : Principal } };
export type RetrieveBtcStatusV2 = { 'Signing' : null } |
  { 'Confirmed' : { 'txid' : Uint8Array | number[] } } |
  { 'Sending' : { 'txid' : Uint8Array | number[] } } |
  { 'AmountTooLow' : null } |
  { 'WillReimburse' : ReimbursementDeposit } |
  { 'Unknown' : null } |
  { 'Submitted' : { 'txid' : Uint8Array | number[] } } |
  { 'Reimbursed' : ReimbursementDeposit } |
  { 'Pending' : null };
export interface _SERVICE {
  'confirm_and_convert_ckbtc' : ActorMethod<[], bigint>,
  'etch_rune' : ActorMethod<[EtchingArgs], [string, string]>,
  'get_btc_balance' : ActorMethod<[], bigint>,
  'get_deposit_address_for_bitcoin' : ActorMethod<[], string>,
  'get_deposit_address_for_ckbtc' : ActorMethod<[], string>,
  'query_converstion_status' : ActorMethod<[bigint], RetrieveBtcStatusV2>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: ({ IDL }: { IDL: IDL }) => IDL.Type[];
