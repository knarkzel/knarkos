use lisp::parse_expr;

fn main() {
    match parse_expr("(+ 1 2 3)") {
        Ok((_, success)) => {
            println!("Parsed following: {success}");
            println!("{success:#?}");
        }
        Err(error) => eprintln!("Got error: {error:?}"),
    }
}
