use std::collections::HashMap;
use crate::parser::Expr;
use crate::parser::Statement;

pub fn eval(expr: Expr, hm: &HashMap<String, i32>) -> Option<i32> {
    match expr {
        Expr::Int(e) => Some(e),
        Expr::Add(a, b) => Some(eval(*a, hm)? + eval(*b, hm)?),
        Expr::Sub(a, b) => Some(eval(*a, hm)? - eval(*b, hm)?),
        Expr::Mul(a, b) => Some(eval(*a, hm)? * eval(*b, hm)?),
        Expr::Div(a, b) => {
            Some(eval(*a, hm)? / eval(*b, hm)?)
        }
        Expr::Var(s) => {
            match hm.get(&*s){
                None => None,
                Some(a) => Some(*a)
            }
        }
    }
}

pub fn eval_statement(stmt: Statement, hm: &mut HashMap<String, i32>) -> Option<i32> {
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