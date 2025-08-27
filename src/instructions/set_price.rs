use pinocchio::{account_info::AccountInfo, instruction::Signer, program_error::ProgramError, sysvars::{clock::Clock, rent::Rent, Sysvar}, *};
use pinocchio_system::instructions::CreateAccount;

use crate::states::{price_account, PriceAccount};

pub fn process_set_price(accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {

    if instruction_data.len() < 16 {
        return Err(ProgramError::InvalidInstructionData);
    }

    let [signer, price_account, system_program] = accounts else {
        return Err(ProgramError::InvalidAccountData);
    };

    let price_account_id = u64::from_le_bytes(
        instruction_data[0..8].try_into().map_err(|_| ProgramError::InvalidInstructionData)?
    );

    let price_to_set = f64::from_le_bytes(
        instruction_data[8..16].try_into().map_err(|_| ProgramError::InvalidInstructionData)?
    );

    let (price_account_pda, _bump) = pubkey::find_program_address(
        &[b"price_feed_account", price_account_id.to_le_bytes().as_ref()],
        &crate::ID
    );

    if !signer.is_signer() || price_account.data_len() == 0 || *price_account.key() != price_account_pda {
        return Err(ProgramError::InvalidAccountData);
    };

    let mut price_account_mut = PriceAccount::from_account_info_mut(price_account)?;
    price_account_mut.price = price_to_set;
    price_account_mut.last_updated_timestamp = Clock::get()?.unix_timestamp;

    Ok(())
}

// =================== TESTING process_set_price =================== 

#[cfg(test)]
mod tests {
    use mollusk_svm::{program, Mollusk, result::Check};
    use solana_sdk::{
        account::Account, instruction::{AccountMeta, Instruction}, program_error::ProgramError, pubkey::Pubkey
    };

    const PROGRAM_ID: Pubkey = solana_sdk::pubkey!("4zSrGy87rYtohmWK7PLBsojskZQa38GMwmoQkeK1nJSD");
    const SIGNER: Pubkey = Pubkey::new_from_array([1u8; 32]);

    #[test]
    fn test_process_set_price() {
        let mollusk = Mollusk::new(&PROGRAM_ID, "target/deploy/pinocchio_price_account");
        
        let price_account_id = 13u64;
        let price_to_set = 100.0f64; 

        let mut instruction_data = vec![];
        instruction_data.push(1u8); 
        instruction_data.extend_from_slice(&price_account_id.to_le_bytes()); 
        instruction_data.extend_from_slice(&price_to_set.to_le_bytes()); 

        let (price_account_pda, _bump) = Pubkey::find_program_address(
            &[b"price_feed_account", price_account_id.to_le_bytes().as_ref()],
            &PROGRAM_ID
        );

        let mock_price_account_data = {
            let mut data = vec![];
            data.extend_from_slice(&price_account_id.to_le_bytes()); // price_account_id
            data.extend_from_slice(&0.0f64.to_le_bytes()); // initial price
            data.extend_from_slice(&0i64.to_le_bytes()); // initial timestamp
            data
        };

        let (system_program_id, system_account) = program::keyed_account_for_system_program();

        let instruction = Instruction {
            program_id: PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(SIGNER, true),
                AccountMeta::new(price_account_pda, false),
                AccountMeta::new(system_program_id, false),
            ],
            data: instruction_data
        };

        let signer_account = Account {
            lamports: 10_000_000,
            data: vec![],
            owner: solana_sdk::system_program::id(),
            executable: false,
            rent_epoch: 0,
        };

        let price_account = Account {
            lamports: 1_000_000, 
            data: mock_price_account_data, 
            owner: PROGRAM_ID, 
            executable: false,
            rent_epoch: 0,
        };

        mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (SIGNER, signer_account),
                (price_account_pda, price_account),
                (system_program_id, system_account),
            ],
            &[Check::success()],
        );
    }

    #[test]
    fn test_create_price_account_insufficient_instruction_data() {
        let mollusk = Mollusk::new(&PROGRAM_ID, "target/deploy/pinocchio_price_account");

        // Only 7 bytes instead of required 8
        let instruction_data = vec![0u8, 123, 0, 0, 0, 0, 0, 0]; // Missing 1 byte

        let (price_account_pda, _) = Pubkey::find_program_address(
            &[b"price_feed_account", 123u64.to_le_bytes().as_ref()],
            &PROGRAM_ID
        );

        let (system_program_id, system_account) = program::keyed_account_for_system_program();

        let instruction = Instruction {
            program_id: PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(SIGNER, true),
                AccountMeta::new(price_account_pda, false),
                AccountMeta::new(system_program_id, false),
            ],
            data: instruction_data,
        };

        let signer_account = Account {
            lamports: 10_000_000,
            data: vec![],
            owner: solana_sdk::system_program::id(),
            executable: false,
            rent_epoch: 0,
        };

        let price_account = Account {
            lamports: 0,
            data: vec![],
            owner: solana_sdk::system_program::id(),
            executable: false,
            rent_epoch: 0,
        };

        mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (SIGNER, signer_account),
                (price_account_pda, price_account),
                (system_program_id, system_account),
            ],
            &[Check::err(ProgramError::InvalidInstructionData)],
        );
    }

    #[test]
    fn test_create_price_account_invalid_pda() {
        let mollusk = Mollusk::new(&PROGRAM_ID, "target/deploy/pinocchio_price_account");

        let price_account_id = 123u64;
        let mut instruction_data = vec![];
        instruction_data.push(0u8);
        instruction_data.extend_from_slice(&price_account_id.to_le_bytes());

        // Use wrong PDA (different seed)
        let (wrong_pda, _) = Pubkey::find_program_address(
            &[b"wrong_seed", price_account_id.to_le_bytes().as_ref()],
            &PROGRAM_ID
        );

        let (system_program_id, system_account) = program::keyed_account_for_system_program();

        let instruction = Instruction {
            program_id: PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(SIGNER, true),
                AccountMeta::new(wrong_pda, false),
                AccountMeta::new(system_program_id, false),
            ],
            data: instruction_data,
        };

        let signer_account = Account {
            lamports: 10_000_000,
            data: vec![],
            owner: solana_sdk::system_program::id(),
            executable: false,
            rent_epoch: 0,
        };

        let price_account = Account {
            lamports: 0,
            data: vec![],
            owner: solana_sdk::system_program::id(),
            executable: false,
            rent_epoch: 0,
        };

        mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (SIGNER, signer_account),
                (wrong_pda, price_account),
                (system_program_id, system_account),
            ],
            &[Check::err(ProgramError::InvalidAccountData)],
        );
    }

    #[test]
    fn test_create_price_account_already_initialized() {
        let mollusk = Mollusk::new(&PROGRAM_ID, "target/deploy/pinocchio_price_account");

        let price_account_id = 123u64;
        let mut instruction_data = vec![];
        instruction_data.push(0u8);
        instruction_data.extend_from_slice(&price_account_id.to_le_bytes());

        let (price_account_pda, _) = Pubkey::find_program_address(
            &[b"price_feed_account", price_account_id.to_le_bytes().as_ref()],
            &PROGRAM_ID
        );

        let (system_program_id, system_account) = program::keyed_account_for_system_program();

        let instruction = Instruction {
            program_id: PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(SIGNER, true),
                AccountMeta::new(price_account_pda, false),
                AccountMeta::new(system_program_id, false),
            ],
            data: instruction_data,
        };

        let signer_account = Account {
            lamports: 10_000_000,
            data: vec![],
            owner: solana_sdk::system_program::id(),
            executable: false,
            rent_epoch: 0,
        };

        // Account already has data (already initialized)
        let already_initialized_account = Account {
            lamports: 1_000_000,
            data: vec![1, 2, 3, 4], // Non-empty data
            owner: PROGRAM_ID,
            executable: false,
            rent_epoch: 0,
        };

        mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (SIGNER, signer_account),
                (price_account_pda, already_initialized_account),
                (system_program_id, system_account),
            ],
            &[Check::err(ProgramError::AccountAlreadyInitialized)],
        );
    }
}