use lisp::{parse, VirtualMachine};

fn main() {
    let ast = parse("(+ 15 (* 3 5))").unwrap();
    let vm = VirtualMachine::new(&ast);
    if let Some(output) = vm.run() {
        println!("Output is {output}");
    }
}
