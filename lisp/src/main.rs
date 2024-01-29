use lisp::{parse_expr, VirtualMachine};

fn main() {
    let (_, ast) = parse_expr("(+ 1 2 3)").unwrap();
    let vm = VirtualMachine::new(&[ast]);
    dbg!(&vm);
}
