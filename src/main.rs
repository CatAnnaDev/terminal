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
        if d == "help"{
println!(r#"    sin(radian) => f64
    cos(radian) => f64
    tan(radian) => f64
    hypot(value, value) => f64
    sqrt(value) => f64
    log(value, base) => f64
    log2(value) => f64
    log10(value) => f64
    abs(value) => f64
    rnd(value) => f64
    facto(value) => f64
    deg2rad(degrees) => radian
    rad2deg(radian) => degrees
    supported signe: + - / * ^ e () .
    var system: var_name=10"#)
        }else {
            let (_, y) = parser::parse_statement(d).unwrap();
            let eval = eval_statement(y, &mut hash);
            println!("Var:\t{:?}", hash);
            print_data(d, format!("{}", eval.unwrap()));
        }
    }
}

fn print_data(data: &str, response: String) {
    println!("CMD:\t{}", data);
    println!("Result:\t{}", response);
    println!("{}", format!("{}", "-".repeat(20)));
}

// rnd(x=1.3*5.6 / 1.1 + 0.78) crash