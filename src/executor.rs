use std::{
    io::{Read, Write},
    num::Wrapping,
};

use crate::{error::ExecutionError, instruction::Instruction};

#[derive(Debug)]
pub struct Executor<'a> {
    instructions: Vec<Instruction>,
    buffer: [Wrapping<u8>; 30_000],
    buffer_ptr: usize,
    stdout: std::io::StdoutLock<'a>,
}

impl Executor<'_> {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            buffer: [Wrapping(0); 30_000],
            buffer_ptr: 0,
            stdout: std::io::stdout().lock(),
        }
    }

    pub fn run(mut self) -> Result<(), ExecutionError> {
        self.fill_loops()?;

        let mut instruction_ptr = 0;

        while instruction_ptr < self.instructions.len() {
            let instruction = &self.instructions[instruction_ptr];

            match instruction {
                Instruction::Right => self.buffer_ptr = (self.buffer_ptr + 1) % 30_000,
                Instruction::Left => self.buffer_ptr = (self.buffer_ptr - 1) % 30_000,
                Instruction::Increment => self.buffer[self.buffer_ptr] += 1,
                Instruction::Decrement => self.buffer[self.buffer_ptr] -= 1,
                Instruction::Output => {
                    write!(self.stdout, "{}", self.buffer[self.buffer_ptr].0 as char)?
                }
                Instruction::Input => {
                    let mut buf = [0; 1];

                    std::io::stdin().read_exact(&mut buf)?;

                    self.buffer[self.buffer_ptr] = Wrapping(buf[0]);
                }
                Instruction::LoopStart(Some(loop_end)) => {
                    if self.buffer[self.buffer_ptr].0 == 0 {
                        instruction_ptr = *loop_end;
                    }
                }
                Instruction::LoopEnd(Some(loop_start)) => {
                    if self.buffer[self.buffer_ptr].0 != 0 {
                        instruction_ptr = *loop_start;
                    }
                }
                _ => {}
            };

            instruction_ptr += 1;
        }

        Ok(())
    }

    fn fill_loops(&mut self) -> Result<(), ExecutionError> {
        let mut instruction_ptr = 0;
        let mut stack = Vec::new();

        while instruction_ptr < self.instructions.len() {
            let instruction = &self.instructions[instruction_ptr];

            match instruction {
                Instruction::LoopStart(_) => {
                    stack.push(instruction_ptr);
                }
                Instruction::LoopEnd(_) => {
                    if let Some(loop_start) = stack.pop() {
                        self.instructions[loop_start] =
                            Instruction::LoopStart(Some(instruction_ptr));
                        self.instructions[instruction_ptr] = Instruction::LoopEnd(Some(loop_start));
                    } else {
                        return Err(ExecutionError::UnexpectedBracket(instruction_ptr));
                    }
                }
                _ => {}
            };

            instruction_ptr += 1;
        }

        if let Some(pos) = stack.pop() {
            return Err(ExecutionError::UnexpectedBracket(pos));
        }

        Ok(())
    }
}
