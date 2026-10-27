use std::rc::Rc;

use crate::chunk::{Chunk, OpCode};
use crate::error::CarlaeError;
use crate::value::Value;

pub struct Vm<'a> {
    chunk: &'a Chunk,
    ip: usize,
    stack: Vec<Value>,
    debug_trace: bool,
}

impl<'a> Vm<'a> {
    pub fn new(chunk: &'a Chunk, debug_trace: bool) -> Self {
        Vm {
            chunk,
            ip: 0,
            stack: Vec::new(),
            debug_trace,
        }
    }

    fn read_op(&mut self) -> OpCode {
        let op = self.chunk.code[self.ip];
        self.ip += 1;
        op
    }

    pub fn read_constant(&self, index: usize) -> &Value {
        &self.chunk.constants[index]
    }

    #[allow(clippy::never_loop)]
    pub fn run(&mut self) -> Result<(), CarlaeError> {
        loop {
            let instruction = self.read_op();

            if self.debug_trace {
                self.stack_trace(&instruction);
                println!("{instruction}");
            }

            match instruction {
                OpCode::Constant(c) => {
                    let val = self.read_constant(c);
                    self.stack.push(val.clone());
                }
                OpCode::True => self.stack.push(Value::Boolean(true)),
                OpCode::False => self.stack.push(Value::Boolean(false)),
                OpCode::None => self.stack.push(Value::None),
                OpCode::Negate => {
                    if let Some(val) = self.stack.pop() {
                        self.stack.push(Value::Number(-val.as_number()?))
                    } else {
                        return Err(CarlaeError::Runtime(format!(
                            "[Line {}]: Missing operand for `-`",
                            self.chunk.lines[self.ip - 1]
                        )));
                    }
                }
                op @ (OpCode::Add | OpCode::Subtract | OpCode::Multiply | OpCode::Divide) => {
                    self.binary_op(&op)?
                }
                OpCode::Return => {
                    if let Some(val) = self.stack.pop() {
                        println!("{val}");
                    }
                    return Ok(());
                }
                op => return Err(CarlaeError::Compile(format!("Unknown opcode: {op}"))),
            }
        }

        Ok(())
    }

    pub fn stack_trace(&self, instruction: &OpCode) {
        print!("    ");
        if self.stack.is_empty() {
            println!("[ ]");
            return;
        }
        for val in self.stack.iter() {
            print!("[ {val} ]");
        }
        println!();
    }

    fn binary_op(&mut self, instruction: &OpCode) -> Result<(), CarlaeError> {
        let right = if let Some(val) = self.stack.pop() {
            val
        } else {
            return Err(CarlaeError::Runtime(format!(
                "[Line {}]: Missing right operand for binary operator",
                self.chunk.lines[self.ip - 1]
            )));
        };
        let left = if let Some(val) = self.stack.pop() {
            val
        } else {
            return Err(CarlaeError::Runtime(format!(
                "[Line {}]: Missing right operand for binary operator",
                self.chunk.lines[self.ip - 1]
            )));
        };

        let res = match instruction {
            OpCode::Add => match (left, right) {
                (
                    x @ (Value::Number(_) | Value::Boolean(_)),
                    y @ (Value::Number(_) | Value::Boolean(_)),
                ) => Value::Number(x.as_number()? + y.as_number()?),
                (Value::String(s), Value::String(t)) => Value::String(Rc::new(format!("{s}{t}"))),
                (x, y) => {
                    return Err(CarlaeError::Runtime(format!(
                        "[Line {}]: Invalid arguments to binary operator +: {x}, {y}",
                        self.chunk.lines[self.ip - 1]
                    )));
                }
            },
            OpCode::Subtract => match (left, right) {
                (
                    x @ (Value::Number(_) | Value::Boolean(_)),
                    y @ (Value::Number(_) | Value::Boolean(_)),
                ) => Value::Number(x.as_number()? - y.as_number()?),
                (x, y) => {
                    return Err(CarlaeError::Runtime(format!(
                        "[Line {}]: Invalid arguments to binary operator -: {x}, {y}",
                        self.chunk.lines[self.ip - 1]
                    )));
                }
            },
            OpCode::Multiply => match (left, right) {
                (
                    x @ (Value::Number(_) | Value::Boolean(_)),
                    y @ (Value::Number(_) | Value::Boolean(_)),
                ) => Value::Number(x.as_number()? * y.as_number()?),
                (Value::Number(n), Value::String(s)) | (Value::String(s), Value::Number(n)) => {
                    if n.fract() == 0.0 {
                        Value::String(Rc::new(s.repeat(n as usize)))
                    } else {
                        return Err(CarlaeError::Runtime(format!(
                            "[Line {}]: Can't multiply strings by fractional numbers",
                            self.chunk.lines[self.ip - 1]
                        )));
                    }
                }
                (x, y) => {
                    return Err(CarlaeError::Runtime(format!(
                        "[Line {}]: Invalid arguments to binary operator *: {x}, {y}",
                        self.chunk.lines[self.ip - 1]
                    )));
                }
            },
            OpCode::Divide => match (left, right) {
                (
                    x @ (Value::Number(_) | Value::Boolean(_)),
                    y @ (Value::Number(_) | Value::Boolean(_)),
                ) => {
                    let y = y.as_number()?;
                    if y == 0.0 {
                        return Err(CarlaeError::Runtime(format!(
                            "[Line {}]: Cannot divide by zero",
                            self.chunk.lines[self.ip - 1]
                        )));
                    }
                    Value::Number(x.as_number()? / y)
                }
                (x, y) => {
                    return Err(CarlaeError::Runtime(format!(
                        "[Line {}]: Invalid arguments to binary operator /: {x}, {y}",
                        self.chunk.lines[self.ip - 1]
                    )));
                }
            },
            op => {
                return Err(CarlaeError::Runtime(format!(
                    "[Line {}]: Invalid binary operator {op}",
                    self.chunk.lines[self.ip - 1]
                )));
            }
        };
        self.stack.push(res);

        Ok(())
    }
}
