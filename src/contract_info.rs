use hedera::{ ContractInfo, ContractId, AccountId, PublicKey };
use mbox::MString;
use crate::{
    timestamp::CTimestamp,
    duration::CDuration,
};

#[repr(C)]
#[derive(Debug)]
pub struct CContractInfo {
    pub contract_id: ContractId,
    pub account_id: AccountId,
    pub contract_account_id: *const libc::c_char,
    pub admin_key: Option<Box<PublicKey>>,
    pub expiration_time: CTimestamp,
    pub auto_renew_period: CDuration,
    pub storage: i64,
}

impl Drop for CContractInfo {
    fn drop(&mut self) {
        if !self.contract_account_id.is_null() {
            unsafe {
                libc::free(self.contract_account_id as _);
            }
        }
    }
}

impl From<ContractInfo> for CContractInfo {
    fn from(contract_info: ContractInfo) -> Self {
        let contract_account_id = MString::from_str(&contract_info.contract_account_id)
            .into_mbox_with_sentinel()
            .into_raw() as _;

        CContractInfo {
            contract_id: contract_info.contract_id,
            account_id: contract_info.account_id,
            contract_account_id,
            admin_key: contract_info.admin_key.map(|t| Box::new(t.into())),
            expiration_time: contract_info.expiration_time.into(),
            auto_renew_period: contract_info.auto_renew_period.into(),
            storage: contract_info.storage
        }
    }
}
