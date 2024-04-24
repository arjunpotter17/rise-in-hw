use borsh::{BorshDeserialize, BorshSerialize};
// use borsh_derive::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CounterAccount {
    pub counter: u32,
}

use crate::instructions::CounterInstructions;
pub mod instructions;

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Counter program entrypoint");

    let instruction = CounterInstructions::unpack(instruction_data)?;

    let account_iter = &mut accounts.iter();

    let accounts = next_account_info(account_iter)?;

    let mut counter_account = CounterAccount::try_from_slice(&accounts.data.borrow())?;

    match instruction {
        CounterInstructions::Increment(args) => {
            msg!("Instruction: Increment");
            counter_account.counter += args.value;
        }
        CounterInstructions::Decrement(args) => {
            msg!("Instruction: Decrement");
            if counter_account.counter < args.value {
                counter_account.counter = 0;
            } else {
                counter_account.counter -= args.value;
            }
        }
        CounterInstructions::Update(args) => {
            msg!("Instruction: Update");
            counter_account.counter = args.value;
        }
        CounterInstructions::Clear => {
            msg!("Instruction: Clear");
            counter_account.counter = 0;
        }
    }

    counter_account.serialize(&mut &mut accounts.data.borrow_mut()[..])?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use solana_program::{clock::Epoch, pubkey::Pubkey};
    use std::mem;

    #[test]
    fn test_counter() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();

        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );

        let accounts = vec![account];

        let mut increment_instruction_data: Vec<u8> = vec![0];
        let mut decrement_instruction_data: Vec<u8> = vec![1];
        let mut update_instruction_data: Vec<u8> = vec![2];
        let reset_instruction_data: Vec<u8> = vec![3];

        let increment_value = 5u32;
        increment_instruction_data.extend_from_slice(&increment_value.to_le_bytes());

        process_instruction(&program_id, &accounts, &increment_instruction_data).unwrap();

        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            5
        );

        let decrement_value = 20u32;
        decrement_instruction_data.extend_from_slice(&decrement_value.to_le_bytes());

        process_instruction(&program_id, &accounts, &decrement_instruction_data).unwrap();

        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );

        let update_value = 33u32;
        update_instruction_data.extend_from_slice(&update_value.to_le_bytes());

        process_instruction(&program_id, &accounts, &update_instruction_data).unwrap();

        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            33
        );

        process_instruction(&program_id, &accounts, &reset_instruction_data).unwrap();

        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );
    }
}
