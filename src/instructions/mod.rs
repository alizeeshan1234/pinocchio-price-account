use pinocchio::program_error::ProgramError;
use shank::ShankInstruction;

pub mod create_price_account;
pub mod modify_price;
pub mod set_price;
pub mod get_price;

#[repr(u8)]
#[derive(ShankInstruction)]
pub enum PriceInstructions {
    #[account(0, writable, signer, name="payer", desc="Account that pays for account creation")]
    #[account(1, writable, name="price_account", desc="The price account to be created")]
    #[account(2, name="system_program", desc="System program")]
    CreatePriceAccount = 0,

    #[account(0, signer, name="signer", desc="Signer authority")]
    #[account(1, writable, name="price_account", desc="The price account to update")]
    #[account(2, name="system_program", desc="System program")]
    SetPrice = 1,

    #[account(0, signer, name="signer", desc="Signer authority")]
    #[account(1, writable, name="price_account", desc="The price account to update")]
    #[account(2, name="system_program", desc="System program")]
    ModifyPrice = 2,

    #[account(0, name="price_account", desc="The price account to read from")]
    GetPrice = 3,
}


impl TryFrom<&u8> for PriceInstructions {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PriceInstructions::CreatePriceAccount),
            1 => Ok(PriceInstructions::SetPrice),
            2 => Ok(PriceInstructions::ModifyPrice),
            3 => Ok(PriceInstructions::GetPrice),
            _ => Err(ProgramError::InvalidInstructionData)
        }
    }
}
