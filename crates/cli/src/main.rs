use ast::{AstNode, XmlElement};
use parser::parse;
use std::{
    io::{stdin, stdout, Write},
    process::exit,
};

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

        println!("Parsed successfully");

        for node in parse.unwrap().descendants() {
            if XmlElement::can_cast(node.kind()) {
                let element = XmlElement::cast(node).unwrap();

                println!("{:?}", element.name_ref().unwrap().syntax().text());
            }
        }
    }
}
