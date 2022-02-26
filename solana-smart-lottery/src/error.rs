use thiserror::Error;

use solana_program::program_error::ProgramError;

#[#[derive(Error, Debug, Copy, Clone)]]
pub enum PlayLotteryError {
    #[error("Invalid Instruction")]
    InvalidInstruction,
}

impl From<PlayLotteryError> for ProgramError {
    fn from(e: PlayLotteryError) -> Self {
        ProgramError::Custom(e as u32)
    }
}