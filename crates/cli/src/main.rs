use ast::{AstNode, View};
use parser::parse;
use std::io::{stdin, stdout, Write};
use std::process::exit;

fn main() {
    loop {
        print!("\nEnter code: ");

        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let parse = parse(&input).ok();

        if parse.is_err() {
            print!("Failed to parse.:");
            println!("{:?}", parse.unwrap_err());
            exit(-1);
        }

        for node in parse.unwrap().descendants() {
            if View::can_cast(node.kind()) {
                println!("{:?}", View::cast(node));
            }
        }

        println!("Parsed successfully");
    }
}
