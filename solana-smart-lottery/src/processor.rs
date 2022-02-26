use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg,
    pubkey::Pubkey,
    program_pack::{Pack, IsInitialized},
    sysvar::{rent::Rent, Sysvar},
};

use crate::instruction::PlayLotteryInstruction;

pub struct Processor;
impl Processor {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], inscruction_data: &[u8]) -> ProgramResult {
        let instruction = PlayLotteryInstruction::unpack(inscruction_data)?;

        match instruction {
            PlayLotteryInstruction::InitPlayLottery { amount } => {
                msg!("Instruction: PlayLottery");
                Self::process_play_lottery(accounts, amount, program_id)
            }
        }
    }

    fn process_play_lottery(
        accounts: &[AccountInfo],
        amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let initializer = next_account_info(account_info_iter)?;

        if !initializer.is_signer {
            return Err(ProgramError::MissingRequiredSignature)
        }

        let temp_token_account = next_account_info(account_info_iter)?;

        let token_to_receive_account = next_account_info(account_info_iter)?;
        if *token_to_receive_account.owner != spl_token::id() {
            return Err(ProgramError::IncorrectProgramId);
        }

        Ok(())
    }
}