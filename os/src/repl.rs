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

pub fn pop() {
    let mut prompt = PROMPT.lock();
    prompt.pop();
}

pub fn push(character: char) {
    match character as u8 {
        0x20..=0x7e => {
            let mut prompt = PROMPT.lock();
            prompt.push(character);
        }
        _ => {}
    }
}

pub fn eval() {
    let mut prompt = PROMPT.lock();
    match parse(prompt.as_str()) {
        Ok(ast) => {
            let vm = VirtualMachine::new(&ast);
            match vm.run() {
                Some(value) => println!("\n{value}"),
                None => println!(),
            }
        },
        Err(error) => println!("\n{error}"),
    }
    prompt.clear();
}

