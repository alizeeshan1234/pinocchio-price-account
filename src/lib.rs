use pinocchio::{account_info::AccountInfo, pubkey::Pubkey, ProgramResult, program_error::ProgramError,*};
use pinocchio_pubkey::*;

use crate::instructions::PriceInstructions;

declare_id!("4zSrGy87rYtohmWK7PLBsojskZQa38GMwmoQkeK1nJSD");

pub mod instructions;
pub mod states;

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {

    let (ix_disc, instruction_data) = instruction_data.split_first().ok_or(ProgramError::InvalidInstructionData)?;

    match PriceInstructions::try_from(ix_disc)? {
        PriceInstructions::CreatePriceAccount => instructions::create_price_account::process_create_price_account(accounts, instruction_data)?,
        PriceInstructions::SetPrice => instructions::set_price::process_set_price(accounts, instruction_data)?,
        PriceInstructions::ModifyPrice => instructions::modify_price::process_modify_price(accounts, instruction_data)?,
        PriceInstructions::GetPrice => {},
    }

    Ok(())
}