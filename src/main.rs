use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use crate::error::CarlaeError;
use crate::scanner::Scanner;

mod error;
mod scanner;
mod token;

fn main() -> Result<(), CarlaeError> {
    let mut args = std::env::args().skip(1);

    if args.len() > 1 {
        Err(CarlaeError::General("Usage: carlae [script]".into()))
    } else if let Some(path) = args.next() {
        run_file(&path)
    } else {
        // Err(CarlaeError::General("No REPL >:(".into()))
        // TODO: Write and call run_prompt()

        // TODO: Make these unit tests
        // run("( **/ \t=-(*) )==# lalalala +=3 #EOF".to_string())
        // run("(123+ 456.000001)==579 #lol".to_string())
        // run(r#"3 "hello" 2"#.to_string())
        // run(r#"drink = "banana_smoothie""#.to_string())
        // run(r#"x = "apple" if y == "yes" else "orange""#.to_string())
        // run("x = 1 + 2\ny = 2 + 3\rz = x * y\r\n".to_string())
        // run("x = (\n1 + (\n2 + 3\n)\n* 3\n)\ny = 4".to_string())
        // run("x = (\n1 + (\\\n2 + 3\n)\n)\ny = 4".to_string())
        run("def func():\n    if True:\n        1\n    else:\n        0".to_string())
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

    for t in scanner.tokens.iter() {
        println!("{t:?}")
    }

    Ok(())
}
