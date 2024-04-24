use borsh::BorshDeserialize;
use borsh_derive::BorshSerialize;
use solana_program::program_error::ProgramError;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct UpdateArgs {
    pub value: u32,
}

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct IncrementArgs {
    pub value: u32,
}

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct DecrementArgs {
    pub value: u32,
}

pub enum CounterInstructions {
    Increment(IncrementArgs),
    Decrement(DecrementArgs),
    Update(UpdateArgs),
    Clear,
}

impl CounterInstructions {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        Ok(match variant {
            0 => Self::Increment(IncrementArgs::try_from_slice(rest)?),
            1 => Self::Decrement(DecrementArgs::try_from_slice(rest)?),
            2 => Self::Update(UpdateArgs::try_from_slice(rest)?),
            3 => Self::Clear,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
