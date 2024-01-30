use lisp::{parse_expr, VirtualMachine};

fn main() {
    let (_, ast) = parse_expr("(+ 15 (* 3 5))").unwrap();
    let vm = VirtualMachine::new(&[ast]);
    if let Some(output) = vm.run() {
        println!("Output is {output}");
    }
}
