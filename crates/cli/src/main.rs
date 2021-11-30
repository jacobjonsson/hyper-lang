use ast::validate;
use parser::parse;
use std::io::{stdin, stdout, Write};

fn main() {
    loop {
        print!("\nEnter code: ");

        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let parse = parse(&input);

        validate(&parse.syntax());

        println!("{}", parse.debug_tree());
    }
}
