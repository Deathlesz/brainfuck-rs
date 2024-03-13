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
        self.optimize();
        self.fill_loops()?;

        let mut instruction_ptr = 0;

        while instruction_ptr < self.instructions.len() {
            let instruction = &self.instructions[instruction_ptr];

            match instruction {
                Instruction::Move(value) => {
                    let buffer_ptr = if *value < 0 {
                        if self.buffer_ptr >= -value as usize {
                            self.buffer_ptr - -value as usize
                        } else {
                            30_000 - (-value as usize - self.buffer_ptr)
                        }
                    } else {
                        (self.buffer_ptr + *value as usize) % 30_000
                    };

                    self.buffer_ptr = buffer_ptr;
                }
                Instruction::Add(value) => {
                    if *value < 0 {
                        self.buffer[self.buffer_ptr] -= Wrapping(-value as u8);
                    } else {
                        self.buffer[self.buffer_ptr] += Wrapping(*value as u8);
                    }
                }
                Instruction::Output => {
                    write!(self.stdout, "{}", self.buffer[self.buffer_ptr].0 as char)?
                }
                Instruction::Input => {
                    // TODO: there's gotta be a better way, right?.. Right?!
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

    fn optimize(&mut self) {
        let mut optimized = Vec::with_capacity(self.instructions.len());

        let mut instruction_ptr = 0;
        let mut previous_instruction: Option<Instruction> = None;
        while instruction_ptr < self.instructions.len() {
            let instruction = self.instructions[instruction_ptr];

            match (previous_instruction, instruction) {
                (None, _) => previous_instruction = Some(instruction),
                (Some(Instruction::Move(value1)), Instruction::Move(value2)) => {
                    previous_instruction = Some(Instruction::Move(value1 + value2));
                }
                (Some(Instruction::Add(value1)), Instruction::Add(value2)) => {
                    previous_instruction = Some(Instruction::Add(value1 + value2));
                }
                _ => {
                    if let Some(previous_instruction) = previous_instruction {
                        optimized.push(previous_instruction);
                    } else {
                        optimized.push(instruction)
                    }
                    previous_instruction = Some(instruction)
                }
            }

            instruction_ptr += 1;
        }

        if let Some(previous_instruction) = previous_instruction {
            optimized.push(previous_instruction);
        }

        self.instructions = optimized;
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
