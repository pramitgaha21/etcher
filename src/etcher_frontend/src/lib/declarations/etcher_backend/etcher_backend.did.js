export const idlFactory = ({ IDL }) => {
  const BitcoinNetwork = IDL.Variant({
    'mainnet' : IDL.Null,
    'regtest' : IDL.Null,
    'testnet' : IDL.Null,
  });
  const InitArgs = IDL.Record({
    'network' : BitcoinNetwork,
    'ckbtc_minter' : IDL.Principal,
    'schnorr_canister' : IDL.Principal,
    'ckbtc_ledger' : IDL.Principal,
  });
  const EtchingArgs = IDL.Record({
    'cap' : IDL.Opt(IDL.Nat),
    'offset_stop' : IDL.Opt(IDL.Nat64),
    'height_stop' : IDL.Opt(IDL.Nat64),
    'turbo' : IDL.Bool,
    'offset_start' : IDL.Opt(IDL.Nat64),
    'premine' : IDL.Opt(IDL.Nat),
    'rune' : IDL.Text,
    'divisibility' : IDL.Nat8,
    'fee_rate' : IDL.Opt(IDL.Nat64),
    'amount' : IDL.Opt(IDL.Nat),
    'height_start' : IDL.Opt(IDL.Nat64),
    'symbol' : IDL.Opt(IDL.Nat32),
  });
  const Account = IDL.Record({
    'owner' : IDL.Principal,
    'subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const ReimbursementReason = IDL.Variant({
    'CallFailed' : IDL.Null,
    'TaintedDestination' : IDL.Record({
      'kyt_fee' : IDL.Nat64,
      'kyt_provider' : IDL.Principal,
    }),
  });
  const ReimbursementDeposit = IDL.Record({
    'account' : Account,
    'mint_block_index' : IDL.Nat64,
    'amount' : IDL.Nat64,
    'reason' : ReimbursementReason,
  });
  const RetrieveBtcStatusV2 = IDL.Variant({
    'Signing' : IDL.Null,
    'Confirmed' : IDL.Record({ 'txid' : IDL.Vec(IDL.Nat8) }),
    'Sending' : IDL.Record({ 'txid' : IDL.Vec(IDL.Nat8) }),
    'AmountTooLow' : IDL.Null,
    'WillReimburse' : ReimbursementDeposit,
    'Unknown' : IDL.Null,
    'Submitted' : IDL.Record({ 'txid' : IDL.Vec(IDL.Nat8) }),
    'Reimbursed' : ReimbursementDeposit,
    'Pending' : IDL.Null,
  });
  return IDL.Service({
    'confirm_and_convert_ckbtc' : IDL.Func([], [IDL.Nat64], []),
    'etch_rune' : IDL.Func([EtchingArgs], [IDL.Text, IDL.Text], []),
    'get_btc_balance' : IDL.Func([], [IDL.Nat64], []),
    'get_deposit_address_for_bitcoin' : IDL.Func([], [IDL.Text], []),
    'get_deposit_address_for_ckbtc' : IDL.Func([], [IDL.Text], ['query']),
    'query_converstion_status' : IDL.Func(
        [IDL.Nat64],
        [RetrieveBtcStatusV2],
        ['composite_query'],
      ),
  });
};
export const init = ({ IDL }) => {
  const BitcoinNetwork = IDL.Variant({
    'mainnet' : IDL.Null,
    'regtest' : IDL.Null,
    'testnet' : IDL.Null,
  });
  const InitArgs = IDL.Record({
    'network' : BitcoinNetwork,
    'ckbtc_minter' : IDL.Principal,
    'schnorr_canister' : IDL.Principal,
    'ckbtc_ledger' : IDL.Principal,
  });
  return [InitArgs];
};
