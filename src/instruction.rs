use crate::error::Error;

#[derive(Debug, PartialEq, Hash)]
pub enum Instruction {
    Right,
    Left,
    Increment,
    Decrement,
    Output,
    Input,
    LoopStart(Option<usize>),
    LoopEnd(Option<usize>),
}

impl TryFrom<u8> for Instruction {
    type Error = Error;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            b'>' => Ok(Self::Right),
            b'<' => Ok(Self::Left),
            b'+' => Ok(Self::Increment),
            b'-' => Ok(Self::Decrement),
            b'.' => Ok(Self::Output),
            b',' => Ok(Self::Input),
            b'[' => Ok(Self::LoopStart(None)),
            b']' => Ok(Self::LoopEnd(None)),
            _ => Err(Error::InvalidInstruction(byte)),
        }
    }
}

impl From<Instruction> for u8 {
    fn from(instruction: Instruction) -> Self {
        match instruction {
            Instruction::Right => b'>',
            Instruction::Left => b'<',
            Instruction::Increment => b'+',
            Instruction::Decrement => b'-',
            Instruction::Output => b'.',
            Instruction::Input => b',',
            Instruction::LoopStart(_) => b'[',
            Instruction::LoopEnd(_) => b']',
        }
    }
}
