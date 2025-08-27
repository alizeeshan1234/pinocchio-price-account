use pinocchio::program_error::ProgramError;

pub mod create_price_account;
pub mod modify_price;
pub mod set_price;

#[repr(u8)]
pub enum PriceInstructions {
   CreatePriceAccount = 0,
   SetPrice = 1,
   ModifyPrice = 2,
//    GetPrice = 3,
}

impl TryFrom<&u8> for PriceInstructions {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PriceInstructions::CreatePriceAccount),
            1 => Ok(PriceInstructions::SetPrice),
            2 => Ok(PriceInstructions::ModifyPrice),
            // 3 => Ok(PriceInstructions::GetPrice),
            _ => Err(ProgramError::InvalidInstructionData)
        }
    }
}
