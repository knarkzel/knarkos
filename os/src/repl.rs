use lisp::{VirtualMachine, parse};
use spin::Mutex;
use lazy_static::lazy_static;
use alloc::string::String;
use crate::println;

lazy_static! {
    pub static ref PROMPT: Mutex<String> = {
        Mutex::new(String::new())
    };
}

pub fn push(character: char) {
    let mut prompt = PROMPT.lock();
    prompt.push(character);
}

pub fn eval() {
    let mut prompt = PROMPT.lock();
    if let Ok(ast) = parse(prompt.as_str()) {
        let vm = VirtualMachine::new(&ast);
        match vm.run() {
            Some(value) => println!("\n{value}"),
            None => println!(),
        }
    }
    prompt.clear();
}

