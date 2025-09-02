use pinocchio::{account_info::AccountInfo, program_error::ProgramError,*};

pub fn process_get_price(accounts: &[AccountInfo]) -> ProgramResult {

    let [signer, price_account, system_program] = accounts else {
        return Err(ProgramError::InvalidAccountData);
    };

    if !signer.is_signer() {
        return Err(ProgramError::InvalidAccountData);
    };



    Ok(())
}