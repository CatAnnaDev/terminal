use std::collections::HashMap;
use std::io::{stdin, stdout, Write};

use parser::ParseError;

use crate::eval::eval_statement;

mod parser;
mod eval;

fn main() -> Result<(), ParseError<'static>> {
    let sin = stdin();
    let mut sou = stdout();
    let mut hash = HashMap::new();

    loop {
        let mut data = String::new();
        print!("🐱🦊>");
        sou.flush().unwrap();
        let _ = sin.read_line(&mut data);
        let d = data.trim();
        let (_, y) = parser::parse_statement(d).unwrap();
        let eval = eval_statement(y, &mut hash);
        println!("Var:\t{:?}", hash);
        print_data(d, format!("{}", eval.unwrap()));
    }
}

fn print_data(data: &str, response: String) {
    println!("CMD:\t{}", data);
    println!("Result:\t{}", response);
    println!("{}", format!("{}", "-".repeat(20)));
}