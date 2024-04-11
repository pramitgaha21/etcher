use candid::CandidType;
use serde::Deserialize;

pub enum BitcoinTransactionType {
    Transfer {},
    Etching {},
}

#[derive(CandidType, Deserialize, Debug)]
pub enum DepositType {
    CKBTC,
    NativeBTC,
}
