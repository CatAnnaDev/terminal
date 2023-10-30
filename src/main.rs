mod parser;
mod eval;

use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use parser::ParseError;
use crate::eval::eval_statement;

fn main() -> Result<(), ParseError<'static>> {
    let sin = stdin();
    let mut sou = stdout();
    let mut hash = HashMap::new();

    loop {
        let mut data = String::new();
        print!("Meow>");
        sou.flush().unwrap();
        let _ = sin.read_line(&mut data);

        let x = parser::parse_statement(data.trim()).unwrap();
        let eval = eval_statement(x.1,&mut hash).expect("").to_string();

        println!("hash: {:?}", hash);
        print_data(data.trim().to_string(),eval);
    }
}

fn print_data(data: String, response: String){
    println!("CMD: {}", data);
    println!("Result: {}", response);
    println!("{}", format!("{}", "-".repeat(20)));
}