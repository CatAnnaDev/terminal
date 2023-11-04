use std::collections::HashMap;

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

        Expr::Call(exp,b) => {
            let b = eval(*b, hm)?;
            match exp.as_str() {
                "encule" => {Some(b)}
                "meow" => {Some(b *2.0)}
                _ => {None}
            }
        },

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