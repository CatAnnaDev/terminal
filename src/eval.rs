use std::collections::HashMap;
use std::f64::consts;

use crate::parser::Expr;
use crate::parser::Statement;

pub fn eval(expr: Expr, hm: &HashMap<String, f64>) -> Option<f64> {
    match expr {
        Expr::Float(e) => Some(e),
        Expr::Add(a, b) => Some(eval(*a, hm)? + eval(*b, hm)?),
        Expr::Sub(a, b) => Some(eval(*a, hm)? - eval(*b, hm)?),
        Expr::Mul(a, b) => Some(eval(*a, hm)? * eval(*b, hm)?),
        Expr::Mod(a, b) => Some(eval(*a, hm)? % eval(*b, hm)?),
        Expr::Pow(a, b) => Some(eval(*a, hm)?.powf(eval(*b, hm)?)),

        Expr::Call(exp, b) => {
            let mut arg = Vec::new();
            for exp in b {
                arg.push(eval(exp, hm)?);
            }
            match exp.as_str() {
                "encule" => { Some(arg[0]) }
                "meow" => { Some(arg[0] * 2.0) }
                "sin" => { Some(arg[0].sin()) }
                "cos" => { Some(arg[0].cos()) }
                "tan" => { Some(arg[0].tan()) }
                "hypot" => { Some(arg[0].hypot(arg[1])) }
                "sqrt" => { Some(arg[0].sqrt()) }
                "log" => { Some(arg[0].log(arg[1])) }
                "log2" => { Some(arg[0].log2()) }
                "log10" => { Some(arg[0].log10()) }
                "abs" => { Some(arg[0].abs()) }
                "rnd" => { Some(arg[0].round()) }
                "facto" => { Some(factorielle(arg[0] as u64)) }
                "deg2rad" => { Some(arg[0] * (consts::PI / 180.0)) }
                "rad2deg" => { Some(arg[0] * (180.0 / consts::PI)) }

                _ => { None }
            }
        }

        Expr::Div(a, b) => {
            let a = eval(*a, hm)?;
            let b = eval(*b, hm)?;
            if b == 0.0 {
                None
            } else {
                Some(a / b)
            }
        }
        Expr::Var(s) => {
            match hm.get(&*s) {
                None => None,
                Some(a) => Some(*a)
            }
        }
    }
}

fn factorielle(nb: u64) -> f64 {
    if nb <= 0 {
        return 1.0;
    }
    return nb as f64 * factorielle(nb - 1);
}

pub fn eval_statement(stmt: Statement, hm: &mut HashMap<String, f64>) -> Option<f64> {
    match stmt {
        Statement::Assign(a, b) => {
            let x = eval(b, hm)?;
            hm.insert(a.clone(), x);
            Some(x)
        }
        Statement::Expr(a) => {
            eval(a, hm)
        }
    }
}