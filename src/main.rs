extern crate core;

use std::collections::HashMap;
use std::env;
use std::env::{current_dir, set_current_dir};
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Command, exit};
use parser::ParseError;
use crate::eval::eval_statement;
mod parser;
mod eval;

fn main() -> Result<(), ParseError<'static>> {
    let sin = stdin();
    let mut sou = stdout();
    let mut hash = HashMap::new();
    let home = env::var("HOME").unwrap();
    let home= Path::new(&home);

    loop {
        let current = current_dir().unwrap();
        let mut data = String::new();
        print!("{}>", current.display());
        sou.flush().unwrap();
        let _ = sin.read_line(&mut data);
        let d = data.trim();

        if let Ok((rest, cmd)) = parser::parse_name(d){
            match cmd.as_str() {
                "help" => {
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
    var system: var_name=10"#);
                }
                "eval" => {
                    match parser::parse_statement(rest) {
                        Ok((_, stat)) => {
                            match eval_statement(stat, &mut hash){
                                Some(eval) => {println!("Var:\t{:?}", hash); print_data(rest, format!("{}", eval))},
                                None => eprintln!("{rest} | Error")
                            }
                        },
                        Err(e) => eprintln!("Bad request: {:?}", e),
                    };
                }
                "cd" => {
                    let new_path = match rest {
                        "" => {
                            home
                        }
                        _ => {
                            Path::new(rest)
                        }
                    };
                    if let Err(e) = set_current_dir(&new_path){
                        eprintln!("{e}");
                    }
                }
                "exit" => {
                    exit(0);
                }
                _ => {
                    let param = if rest.is_empty(){ vec![] }else { rest.split(" ").collect::<Vec<&str>>() };
                    run_process(&cmd, param.as_slice());
                }
            }
        }
    }
}

fn run_process(process_name: &str, args: &[&str]){
    match Command::new(process_name).args(args).status() {
        Ok(_) => {}
        Err(e) => {eprintln!("{e}")}
    }
}

fn print_data(data: &str, response: String) {
    println!("CMD:\t{}", data);
    println!("Result:\t{}", response);
    println!("{}", format!("{}", "-".repeat(30)));
}