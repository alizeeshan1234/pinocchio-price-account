use pinocchio::{account_info::AccountInfo, instruction::Signer, program_error::ProgramError, sysvars::{clock::Clock, rent::Rent, Sysvar}, *};
use pinocchio_system::instructions::CreateAccount;

use crate::states::{PriceAccount};

pub fn process_create_price_account(accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {

    if accounts.len() < 3 || instruction_data.len() < 8 {
        return Err(ProgramError::InvalidAccountData);
    };

    let [signer, price_account, _system_program] = accounts else {
        return Err(ProgramError::InvalidAccountData);
    };

    if !signer.is_signer() {
        return Err(ProgramError::InvalidAccountData);
    };

    if price_account.data_len() != 0 {
        return Err(ProgramError::AccountAlreadyInitialized);
    };

    let price_account_id = u64::from_le_bytes(
        instruction_data[0..8].try_into().map_err(|_| ProgramError::InvalidInstructionData)?
    );

    let (price_account_pda, bump) = pubkey::find_program_address(
        &[b"price_feed_account", price_account_id.to_le_bytes().as_ref()],
        &crate::ID
    );

    if *price_account.key() != price_account_pda {
        return Err(ProgramError::InvalidAccountData);
    };

    let price_account_id_clone = price_account_id.to_le_bytes();

    let bump_arr = [bump];
    let seeds = seeds!(
        b"price_feed_account",
        price_account_id_clone.as_ref(),
        &bump_arr
    );

    CreateAccount {
        from: signer,
        to: price_account,
        lamports: Rent::get()?.minimum_balance(PriceAccount::SIZE),
        space: PriceAccount::SIZE as u64,
        owner: &crate::ID
    }.invoke_signed(&[Signer::from(&seeds)])?;

    let mut price_account_mut = PriceAccount::from_account_info_mut(price_account)?;
    price_account_mut.price = 0.0;
    price_account_mut.last_updated_timestamp = Clock::get()?.unix_timestamp;
    price_account_mut.price_account_bump = bump;

    Ok(())
}

// =================== TESTING process_create_price_account =================== 

#[cfg(test)]
mod tests {
    use mollusk_svm::{program, Mollusk, result::Check};
    use solana_sdk::{
        account::Account,
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    };

    const PROGRAM_ID: Pubkey = solana_sdk::pubkey!("4zSrGy87rYtohmWK7PLBsojskZQa38GMwmoQkeK1nJSD");
    const SIGNER: Pubkey = Pubkey::new_from_array([1u8; 32]);

    #[test]
    fn test_process_create_price_account() {
        let mollusk = Mollusk::new(&PROGRAM_ID, "target/deploy/pinocchio_price_account");

        let price_account_id = 13u64;
        
        let mut instruction_data = vec![];
        instruction_data.push(0u8); 
        instruction_data.extend_from_slice(&price_account_id.to_le_bytes()); 
        
        println!("Full instruction_data: {:?}", instruction_data);

        let (price_account_pda, _bump) = Pubkey::find_program_address(
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
            &[Check::success()],
        );
    }

    #[test]
    fn test_process_create_price_account_invalid_pda() {
        let mollusk = Mollusk::new(&PROGRAM_ID, "target/deploy/price_account");

        let price_account_id = 123u64;
        
        let mut instruction_data = vec![];
        instruction_data.push(0u8); 
        instruction_data.extend_from_slice(&price_account_id.to_le_bytes()); 
        
        println!("Full instruction_data: {:?}", instruction_data);

        let (price_account_pda, _bump) = Pubkey::find_program_address(
            &[b"price_account", price_account_id.to_le_bytes().as_ref()],
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
            &[Check::success()],
        );
    }

     #[test]
    fn test_process_create_price_account_invalid_signer() {
        let mollusk = Mollusk::new(&PROGRAM_ID, "target/deploy/price_account");

        let price_account_id = 123u64;
        
        let mut instruction_data = vec![];
        instruction_data.push(0u8); 
        instruction_data.extend_from_slice(&price_account_id.to_le_bytes()); 
        
        println!("Full instruction_data: {:?}", instruction_data);

        let (price_account_pda, _bump) = Pubkey::find_program_address(
            &[b"price_feed_account", price_account_id.to_le_bytes().as_ref()],
            &PROGRAM_ID
        );

        let (system_program_id, system_account) = program::keyed_account_for_system_program();

        let instruction = Instruction {
            program_id: PROGRAM_ID,
            accounts: vec![
                AccountMeta::new(SIGNER, false),
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
            &[Check::success()],
        );
    }
}