use std::collections::HashMap;
use std::rc::Rc;

use crate::chunk::{Chunk, OpCode};
use crate::error::CarlaeError;
use crate::value::{self, Value};

pub struct Vm<'a> {
    chunk: &'a Chunk,
    ip: usize,
    stack: Vec<Value>,
    globals: HashMap<Rc<String>, Value>,
    debug_trace: bool,
}

impl<'a> Vm<'a> {
    pub fn new(chunk: &'a Chunk, debug_trace: bool) -> Self {
        Vm {
            chunk,
            ip: 0,
            stack: Vec::new(),
            globals: HashMap::new(),
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
                OpCode::Not => {
                    if let Some(val) = self.stack.pop() {
                        self.stack.push(Value::Boolean(!val.is_truthy()))
                    } else {
                        return Err(CarlaeError::Runtime(format!(
                            "[Line {}]: Missing operand for `not`",
                            self.chunk.lines[self.ip - 1]
                        )));
                    }
                }
                op @ (OpCode::Add
                | OpCode::Subtract
                | OpCode::Multiply
                | OpCode::Divide
                | OpCode::Equal
                | OpCode::Greater
                | OpCode::Less) => self.binary_op(&op)?,
                op @ (OpCode::Print(n) | OpCode::Println(n)) => {
                    let first_arg = self
                        .stack
                        .len()
                        .checked_sub(n)
                        .expect("All values to print present in stack.");
                    self.print_op(&op, first_arg)?
                }
                OpCode::AssignGlobal(c) => self.assign_global(c)?,
                OpCode::GetGlobal(c) => self.get_global(c)?,
                OpCode::JumpIfFalse(i) => self.jump(i)?,
                OpCode::Pop => _ = self.stack.pop(),
                OpCode::Return => return Ok(()),
                op => return Err(CarlaeError::Runtime(format!("Unknown opcode: {op}"))),
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
            OpCode::Equal => match (left, right) {
                (Value::None, Value::None) => Value::Boolean(true),
                (Value::None, _) | (_, Value::None) => Value::Boolean(false),
                (Value::String(s), Value::String(t)) => Value::Boolean(s == t),
                (x, y) => Value::Boolean(x.as_number()? == y.as_number()?),
            },
            OpCode::Greater => match (left, right) {
                (Value::None, _) | (_, Value::None) => {
                    return Err(CarlaeError::Runtime(format!(
                        "[Line {}]: Invalid argument to binary operator >: None",
                        self.chunk.lines[self.ip - 1]
                    )));
                }
                (Value::String(s), Value::String(t)) => Value::Boolean(s > t),
                (x, y) => Value::Boolean(x.as_number()? > y.as_number()?),
            },
            OpCode::Less => match (left, right) {
                (Value::None, _) | (_, Value::None) => {
                    return Err(CarlaeError::Runtime(format!(
                        "[Line {}]: Invalid argument to binary operator <: None",
                        self.chunk.lines[self.ip - 1]
                    )));
                }
                (Value::String(s), Value::String(t)) => Value::Boolean(s < t),
                (x, y) => Value::Boolean(x.as_number()? < y.as_number()?),
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

    fn print_op(&mut self, op: &OpCode, start_idx: usize) -> Result<(), CarlaeError> {
        for i in start_idx..self.stack.len() {
            let val = self
                .stack
                .get(i)
                .expect("Value to print present in stack at given index");
            if i != start_idx {
                print!(" ");
            }
            print!("{val}");
        }
        self.stack.truncate(start_idx);
        if matches!(op, OpCode::Println(_)) {
            println!();
        }

        Ok(())
    }

    fn assign_global(&mut self, index: usize) -> Result<(), CarlaeError> {
        let name = match self.read_constant(index) {
            Value::String(s) => s.clone(),
            _ => {
                return Err(CarlaeError::Runtime(format!(
                    "[Line {}]: Invalid name for variable assignment",
                    self.chunk.lines[self.ip - 1]
                )));
            }
        };
        let value = if let Some(val) = self.stack.pop() {
            val
        } else {
            return Err(CarlaeError::Runtime(format!(
                "[Line {}]: Missing value for variable assignment",
                self.chunk.lines[self.ip - 1]
            )));
        };
        self.globals.insert(name, value);
        Ok(())
    }

    fn get_global(&mut self, index: usize) -> Result<(), CarlaeError> {
        let name = match self.read_constant(index) {
            Value::String(s) => s,
            _ => {
                return Err(CarlaeError::Runtime(format!(
                    "[Line {}]: Invalid name for variable assignment",
                    self.chunk.lines[self.ip - 1]
                )));
            }
        };
        match self.globals.get(name) {
            Some(v) => self.stack.push(v.clone()),
            None => {
                return Err(CarlaeError::Runtime(format!(
                    "[Line {}]: Variable `{name}` does not exist",
                    self.chunk.lines[self.ip - 1]
                )));
            }
        };
        Ok(())
    }

    fn jump(&mut self, to: usize) -> Result<(), CarlaeError> {
        if self
            .stack
            .last()
            .expect("Value from if condition on top of stack")
            .is_falsey()
        {
            self.ip = to;
        }
        Ok(())
    }
}
