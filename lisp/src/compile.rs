use crate::{Atom, Expr, Operator};

#[derive(Debug, Clone)]
pub enum Instruction {
    Add,
    Push(usize),
}

#[derive(Debug, Clone)]
pub struct VirtualMachine {
    stack: Vec<usize>,
    code: Vec<Instruction>,
}

impl VirtualMachine {
    /// Takes AST and compiles it into code
    pub fn new(ast: &[Expr]) -> Self {
        let mut code = Vec::new();
        for expr in ast {
            match expr {
                Expr::Constant(Atom::Number(number)) => {
                    code.push(Instruction::Push(*number));
                }
                Expr::Call(function, args) => {
                    // Push all constants to code
                    for arg in args {
                        match arg {
                            Expr::Constant(Atom::Number(number)) => {
                                code.push(Instruction::Push(*number));
                            },
                            _ => panic!("Invalid argument: {arg}"),
                        }
                    }

                    // Push (n - 1) function instruction
                    let instruction = match function.as_ref() {
                        Expr::Constant(Atom::Operator(Operator::Plus)) => {
                            Instruction::Add
                        },
                        _ => panic!("Invalid function: {function}"),
                    };
                    for _ in 0..args.len() - 1 {
                        code.push(instruction.clone());
                    }
                },
                _ => panic!("Invalid expression: {expr}"),
            }
        }
        Self {
            code,
            stack: Vec::new(),
        }
    }

    pub fn run(&mut self) -> Option<usize> {
        self.stack = Vec::new();
        for instruction in &self.code {
            match instruction {
                Instruction::Add => {
                    if let (Some(a), Some(b)) = (self.stack.pop(), self.stack.pop()) {
                        let output = a + b;
                        self.stack.push(output);
                    } else {
                        panic!("Stack underflow");
                    }
                },
                Instruction::Push(number) => {
                    self.stack.push(*number);
                },
            }
        }
        self.stack.pop()
    }
}
