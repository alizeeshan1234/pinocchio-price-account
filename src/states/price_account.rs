use pinocchio::{account_info::{AccountInfo, Ref, RefMut}, program_error::ProgramError, *};
use shank::ShankAccount;

#[derive(Debug, Clone, Copy, PartialEq, ShankAccount)]
pub struct PriceAccount {
    pub price: f64,
    pub last_updated_timestamp: i64,
    pub price_account_bump: u8,
}

impl PriceAccount {
    pub const SIZE: usize = core::mem::size_of::<PriceAccount>();

    pub fn from_account_info(accounts: &AccountInfo) -> Result<Ref<Self>, ProgramError> {
        if accounts.data_len() < PriceAccount::SIZE {
            return Err(ProgramError::InvalidAccountData);
        };

        Ok(Ref::map(accounts.try_borrow_data()?, |data| unsafe {
            &*(data.as_ptr() as *const Self)
        }))
    }

    pub fn from_account_info_mut(accounts: &AccountInfo) -> Result<RefMut<Self>, ProgramError> {
        if accounts.data_len() < PriceAccount::SIZE {
            return Err(ProgramError::InvalidAccountData);
        };

        Ok(RefMut::map(accounts.try_borrow_mut_data()?, |data| unsafe {
            &mut *(data.as_mut_ptr() as *mut Self)
        }))
    }

}