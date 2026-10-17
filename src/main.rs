#![allow(unused)]

use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use crate::chunk::{Chunk, OpCode};
use crate::error::CarlaeError;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::vm::Vm;

mod builtins;
mod callable;
mod chunk;
mod environment;
mod error;
mod eval;
mod exec;
mod expr;
mod interpreter;
mod parser;
mod scanner;
mod stmt;
mod token;
mod value;
mod vm;

fn main() -> Result<(), CarlaeError> {
    let mut args = std::env::args().skip(1);

    if args.len() > 1 {
        Err(CarlaeError::General("Usage: carlae [script]".into()))
    } else if let Some(path) = args.next() {
        run_file(&path)
    } else {
        Err(CarlaeError::General("No REPL >:(".into()))
        // TODO: Write and call run_prompt()
    }
}

fn run_file(path: impl AsRef<Path>) -> Result<(), CarlaeError> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut source = String::new();

    // TODO: Replace with one-liner?
    if let Err(e) = reader.read_to_string(&mut source) {
        Err(CarlaeError::Io(e))
    } else {
        run(source)
    }
}

fn run(source: String) -> Result<(), CarlaeError> {
    let mut scanner = Scanner::new(source);
    scanner.scan_tokens()?;
    // TODO: Print tokens and AST via a --debug flag
    // for t in scanner.tokens.iter() {
    //     println!("{t:?}")
    // }

    let mut parser = Parser::new(scanner.tokens);
    let program = parser.parse()?;
    // for stmt in program.iter() {
    //     println!("{stmt}")
    // }

    // let mut interpreter = Interpreter::new();
    // interpreter.interpret(&program)?;

    let mut chunk = Chunk::new();
    chunk.add_op(OpCode::OpReturn, 8);
    chunk.add_op(OpCode::OpReturn, 1235);
    chunk.add_op(OpCode::OpConstant(1), 1235);
    chunk.add_constant(0.1);
    chunk.add_constant(88.8);

    print!("{chunk}");

    let mut vm = Vm::new(&chunk);

    Ok(())
}
