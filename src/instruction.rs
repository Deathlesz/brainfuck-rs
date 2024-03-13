use crate::error::Error;

#[derive(Debug, PartialEq, Hash, Clone, Copy)]
pub enum Instruction {
    Move(isize),
    Add(i16),
    Output,
    Input,
    LoopStart(Option<usize>),
    LoopEnd(Option<usize>),
}

impl TryFrom<u8> for Instruction {
    type Error = Error;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            b'>' => Ok(Self::Move(1)),
            b'<' => Ok(Self::Move(-1)),
            b'+' => Ok(Self::Add(1)),
            b'-' => Ok(Self::Add(-1)),
            b'.' => Ok(Self::Output),
            b',' => Ok(Self::Input),
            b'[' => Ok(Self::LoopStart(None)),
            b']' => Ok(Self::LoopEnd(None)),
            _ => Err(Error::InvalidInstruction(byte)),
        }
    }
}
