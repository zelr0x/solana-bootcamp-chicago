use borsh::{BorshDeserialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::instruction::EchoInstruction;

pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = EchoInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;
        match instruction {
            EchoInstruction::Echo{data} => {
                msg!("Instruction: Echo");
                let accounts_iter = &mut accounts.iter();
                let echo_ai = next_account_info(accounts_iter)?;
                if !echo_ai.is_writable {
                    return Err(ProgramError::InvalidAccountData);
                }
                let buf = &mut echo_ai.data.borrow_mut();
                let mut n = buf.len();
                msg!("N = {}", n);
                if is_not_empty(buf) {
                    return Err(ProgramError::AccountAlreadyInitialized);
                }
                if data.len() < n {
                    n = data.len();
                }
                buf[..n].copy_from_slice(&data[..n]);
            },
            _ => {
                unimplemented!("todo");
            }
        }
        Ok(())
    }
}

#[inline]
fn is_not_empty(buf: &[u8]) -> bool {
    !is_empty(buf)
}

#[inline]
fn is_empty(buf: &[u8]) -> bool {
    buf.iter().all(|b| *b == 0x0)
}