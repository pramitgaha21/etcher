use candid::CandidType;
use ordinals::{Etching, Runestone, Terms};
use serde::Deserialize;

#[derive(CandidType, Deserialize, Debug)]
pub enum PaymentType {
    CKBTC,
    NativeBTC,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum EtchingError {
    AnonymousCaller,
    NotEnoughBalance {
        mininum_balance_needed: u128,
        current_balance: u128,
    },
}

#[derive(CandidType, Deserialize, Debug)]
pub struct CandidTerms {
    pub cap: Option<u128>,
    pub amount: Option<u128>,
    pub height: (Option<u64>, Option<u64>),
    pub offset: (Option<u64>, Option<u64>),
}

impl Into<Terms> for CandidTerms {
    fn into(self) -> Terms {
        Terms {
            amount: self.amount,
            cap: self.cap,
            height: self.height,
            offset: self.offset,
        }
    }
}

#[derive(CandidType, Deserialize, Debug)]
pub struct CandidEtching {
    pub divisibility: Option<u8>,
    pub rune: Option<String>,
    pub premine: Option<u128>,
    pub terms: CandidTerms,
    pub symbol: Option<u32>,
}

impl Into<Runestone> for CandidEtching {
    fn into(self) -> Runestone {
        let symbol = match self.symbol {
            None => None,
            Some(symbol) => Some(char::from_u32(symbol).unwrap()),
        };
        let etching = Etching {
            terms: Some(self.terms.into()),
            divisibility: self.divisibility,
            premine: self.premine,
            symbol,
            rune: None,
            spacers: None,
        };
        Runestone {
            edicts: vec![],
            etching: Some(etching),
            mint: None,
            pointer: Some(3),
        }
    }
}
