use candid::{CandidType, Principal};
use icrc_ledger_types::icrc1::account::Account;
use serde::Deserialize;

#[derive(Debug)]
pub struct CkBTCMinter(Principal);

#[derive(CandidType, Debug)]
pub struct RetrieveBtcArgs {
    pub address: String,
    pub amount: u64,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct RetrieveBtcOk {
    pub block_index: u64,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum RetrieveBtcError {
    MalformedAddress(String),
    AlreadyProcessing,
    AmountTooLow(u64),
    InsufficientFunds {
        balance: u64,
    },
    TemporarilyUnavailable(String),
    GenericError {
        error_message: String,
        error_code: u64,
    },
}

#[derive(CandidType, Debug)]
pub struct RetrieveBtcStatusArgs {
    pub block_index: u64,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum ReimbursementReason {
    CallFailed,
    TaintedDestination {
        kyt_fee: u64,
        kyt_provider: Principal,
    },
}

#[derive(CandidType, Deserialize, Debug)]
pub struct ReimbursementRequest {
    pub account: Account,
    pub amount: u64,
    pub reason: ReimbursementReason,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct ReimbursementDeposit {
    pub account: Account,
    pub mint_block_index: u64,
    pub amount: u64,
    pub reason: ReimbursementReason,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum RetrieveBtcStatusV2 {
    Unknown,
    Pending,
    Signing,
    Sending { txid: Vec<u8> },
    Submitted { txid: Vec<u8> },
    AmountTooLow,
    Confirmed { txid: Vec<u8> },
    Reimbursed(ReimbursementDeposit),
    WillReimburse(ReimbursementDeposit),
}

impl CkBTCMinter {
    pub fn new(principal: Principal) -> Self {
        Self(principal)
    }

    pub async fn retrieve_btc(
        &self,
        retrieve_btc_args: RetrieveBtcArgs,
    ) -> Result<RetrieveBtcOk, RetrieveBtcError> {
        ic_cdk::call::<(RetrieveBtcArgs,), (Result<RetrieveBtcOk, RetrieveBtcError>,)>(
            self.0,
            "retrieve_btc",
            (retrieve_btc_args,),
        )
        .await
        .unwrap()
        .0
    }

    pub async fn retrieve_btc_status_v2(&self, arg: RetrieveBtcStatusArgs) -> RetrieveBtcStatusV2 {
        ic_cdk::call::<(RetrieveBtcStatusArgs,), (RetrieveBtcStatusV2,)>(
            self.0,
            "retrieve_btc_status_v2",
            (arg,),
        )
        .await
        .unwrap()
        .0
    }
}

#[derive(Debug)]
pub struct CkBTC(Principal);

impl CkBTC {
    pub fn new(principal: Principal) -> Self {
        Self(principal)
    }

    pub async fn get_balance_of(&self, of: Account) -> u128 {
        ic_cdk::call::<(Account,), (u128,)>(self.0, "icrc1_balance_of", (of,))
            .await
            .unwrap()
            .0
    }
}
