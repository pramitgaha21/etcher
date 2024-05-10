use std::fmt::Display;

use candid::{CandidType, Nat, Principal};
use hex::ToHex;
use icrc_ledger_types::{
    icrc1::{
        account::{Account, Subaccount},
        transfer::{TransferArg, TransferError},
    },
    icrc2::approve::{ApproveArgs, ApproveError},
};
use serde::Deserialize;

#[derive(Debug)]
pub struct CkBTCMinter(Principal);

#[derive(CandidType, Debug)]
pub struct RetrieveBtcArgs {
    pub address: String,
    pub amount: u64,
}

#[derive(CandidType, Debug)]
pub struct RetrieveBtcWithApprovalArgs {
    pub address: String,
    pub amount: u64,
    pub from_subaccount: Option<Subaccount>,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct RetrieveBtcOk {
    pub block_index: u64,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum RetrieveBtcWithApprovalError {
    MalformedAddress(String),
    AlreadyProcessing,
    AmountTooLow(u64),
    InsufficientFunds {
        balance: u64,
    },
    InsufficientAllowance {
        allowance: u64,
    },
    TemporarilyUnavailable(String),
    GenericError {
        error_message: String,
        error_code: u64,
    },
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

impl Display for ReimbursementReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CallFailed => write!(f, "Call Failed"),
            Self::TaintedDestination {
                kyt_fee: _,
                kyt_provider: _,
            } => write!(f, "Tainted Destination"),
        }
    }
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

impl RetrieveBtcStatusV2 {
    pub fn to_string(&self) -> String {
        match self {
            Self::Unknown => "Unknown".to_string(),
            Self::Pending => "Pending".to_string(),
            Self::Signing => "Signing".into(),
            Self::Sending { txid } => format!("Sending, Txid: {}", txid.encode_hex::<String>()),
            Self::Submitted { txid } => format!("Submitted, Txid: {}", txid.encode_hex::<String>()),
            Self::AmountTooLow => "Amount too Low".into(),
            Self::Confirmed { txid } => format!("Confirmed, Txid: {}", txid.encode_hex::<String>()),
            Self::Reimbursed(deposit) => format!("Reimbursed, Reason: {}", deposit.reason),
            Self::WillReimburse(deposit) => format!("Will Reimburse, Reason: {}", deposit.reason),
        }
    }
}

#[derive(CandidType, Debug)]
pub struct EstimateWithdrawalFeeArg {
    pub amount: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct EstimateWithdrawalFeeResponse {
    pub bitcoin_fee: u64,
    pub minter_fee: u64,
}

impl CkBTCMinter {
    pub fn new(principal: Principal) -> Self {
        Self(principal)
    }

    pub async fn estimate_withdrawal_fee(
        &self,
        amount: Option<u64>,
    ) -> EstimateWithdrawalFeeResponse {
        ic_cdk::call::<(EstimateWithdrawalFeeArg,), (EstimateWithdrawalFeeResponse,)>(
            self.0,
            "estimate_withdrawal_fee",
            (EstimateWithdrawalFeeArg { amount },),
        )
        .await
        .unwrap()
        .0
    }

    pub async fn get_deposit_fee(&self) -> u64 {
        ic_cdk::call::<(), (u64,)>(self.0, "get_deposit_fee", ())
            .await
            .unwrap()
            .0
    }

    pub async fn get_withdrawal_account(&self) -> Account {
        ic_cdk::call::<(), (Account,)>(self.0, "get_withdrawal_account", ())
            .await
            .unwrap()
            .0
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

    pub async fn retrieve_btc_with_approval(
        &self,
        arg: RetrieveBtcWithApprovalArgs,
    ) -> Result<RetrieveBtcOk, RetrieveBtcWithApprovalError> {
        ic_cdk::call::<
            (RetrieveBtcWithApprovalArgs,),
            (Result<RetrieveBtcOk, RetrieveBtcWithApprovalError>,),
        >(self.0, "retrieve_btc_with_approval", (arg,))
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

    pub async fn icrc1_transfer(&self, to: Account, amount: u128) -> Result<Nat, TransferError> {
        let arg = TransferArg {
            from_subaccount: None,
            to,
            fee: Some(Nat::from(10u128)),
            memo: None,
            amount: Nat::from(amount - 10),
            created_at_time: None,
        };
        ic_cdk::call::<(TransferArg,), (Result<Nat, TransferError>,)>(
            self.0,
            "icrc1_transfer",
            (arg,),
        )
        .await
        .unwrap()
        .0
    }

    pub async fn icrc2_approve(&self, spender: Account, amount: u128) -> Result<Nat, ApproveError> {
        let arg = ApproveArgs {
            from_subaccount: None,
            spender,
            created_at_time: None,
            expires_at: None,
            amount: Nat::from(amount - 10),
            memo: None,
            expected_allowance: None,
            fee: Some(Nat::from(10u128)),
        };
        ic_cdk::call::<(ApproveArgs,), (Result<Nat, ApproveError>,)>(
            self.0,
            "icrc2_approve",
            (arg,),
        )
        .await
        .unwrap()
        .0
    }
}
