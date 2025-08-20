use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::{next_account_info, AccountInfo}, entrypoint:: ProgramResult, program::invoke, program_error::ProgramError, pubkey::Pubkey, rent::Rent, system_instruction, sysvar::{instructions, Sysvar}, entrypoint};



#[derive(BorshSerialize, BorshDeserialize)]
struct CounterState {
    count: u32,
}

#[derive(BorshSerialize, BorshDeserialize)]
enum CounterInstruction {
  Initialize,
  Double,
  Halve
}

entrypoint!(process_instruction);
pub fn process_instruction(
program_id: &Pubkey,
accounts: &[AccountInfo],
instruction_data: &[u8],
) -> ProgramResult {
    let instruction = CounterInstruction::try_from_slice(instruction_data).map_err(|_| ProgramError::InvalidInstructionData)?;
    
    match instruction {
        CounterInstruction::Initialize => {
            let mut iter = accounts.iter();
            let payer = next_account_info(&mut iter)?;
            let data_account = next_account_info(&mut iter)?;
            let system_program = next_account_info(&mut iter)?;

            if !payer.is_signer {
                return Err(ProgramError::MissingRequiredSignature);  
            }

            let space = 4;
            let rent = Rent::get()?;
            let lamports = rent.minimum_balance(space);

            let create_account_ix = system_instruction::create_account(payer.key, data_account.key, lamports, space as u64, program_id);

            invoke( &create_account_ix, &[ payer.clone(), data_account.clone(), system_program.clone()])?;

            let counter_state = CounterState {
                count: 1,
            };

            counter_state.serialize(&mut *data_account.data.borrow_mut())?;

        }
        CounterInstruction::Double => {
            let mut iter = accounts.iter();
            let account_data = next_account_info(&mut iter)?;

            let mut counter_state = CounterState::try_from_slice(&account_data.data.borrow())?;
            counter_state.count = counter_state.count * 2;
            counter_state.serialize(&mut *account_data.data.borrow_mut())?;


        }
        CounterInstruction::Halve => {
            let mut iter = accounts.iter();
            let account_data = next_account_info(&mut iter)?;

            let mut counter_state = CounterState::try_from_slice(&account_data.data.borrow())?;
            counter_state.count = counter_state.count /2 ;
            counter_state.serialize(&mut *account_data.data.borrow_mut())?;
        }
    }
    Ok(())

}