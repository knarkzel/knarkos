use lisp::{parse_expr, VirtualMachine};

fn main() {
    let (_, ast) = parse_expr("(+ 1 2 3)").unwrap();
    let mut vm = VirtualMachine::new(&[ast]);
    let output = vm.run();
    println!("Output is {output:?}");
}
