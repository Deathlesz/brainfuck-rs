use std::path::PathBuf;

use crate::{error::Error, executor::Executor, instruction::Instruction};

mod error;
mod executor;
mod instruction;

fn main() {
    let path = &std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| exit_with_error(Error::SourceFileError("no source file specified")));

    if !path.exists() || !path.is_file() {
        exit_with_error(Error::SourceFileError(
            "path is not a file or it doesn't exist",
        ));
    }

    let instructions = std::fs::read(path)
        .unwrap_or_else(|_| exit_with_error(Error::SourceFileError("failed to read file")))
        .iter()
        .filter_map(|v| match v {
            b'>' => Some(Instruction::Right),
            b'<' => Some(Instruction::Left),
            b'+' => Some(Instruction::Increment),
            b'-' => Some(Instruction::Decrement),
            b'.' => Some(Instruction::Output),
            b',' => Some(Instruction::Input),
            b'[' => Some(Instruction::LoopStart(None)),
            b']' => Some(Instruction::LoopEnd(None)),
            _ => None,
        })
        .collect();

    let executor = Executor::new(instructions);

    executor.run().unwrap_or_else(|e| exit_with_error(e.into()));
}

fn exit_with_error(error: Error) -> ! {
    eprintln!("{}", error);

    std::process::exit(1);
}
