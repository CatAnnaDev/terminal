#[derive(Debug)]
pub enum ParseError<'a> {
    Empty,
    InvalidChar(char),
    InvalidSequence(&'a str),
}

#[derive(Debug)]
pub(crate) enum Expr {
    Float(f64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Var(String),
}

#[derive(Debug)]
pub(crate) enum Statement {
    Assign(String, Expr),
    Expr(Expr),
}

fn any_char(input: &str) -> Result<(&str, char), ParseError<'_>> {
    match input.chars().next() {
        None => Err(ParseError::Empty),
        Some(c) => Ok((&input[c.len_utf8()..], c)),
    }
}

fn satisfy<F>(f: F, input: &str) -> Result<(&str, char), ParseError<'_>>
    where
        F: FnOnce(char) -> bool,
{
    match input.chars().next() {
        None => Err(ParseError::Empty),
        Some(c) => {
            if f(c) {
                Ok((&input[c.len_utf8()..], c))
            } else {
                Err(ParseError::InvalidChar(c))
            }
        }
    }
}

fn take_while<F>(f: F, input: &str) -> Result<(&str, &str), ParseError<'_>>
    where
        F: Fn(char) -> bool,
{
    let mut index = 0;
    loop {
        match any_char(&input[index..]) {
            Err(_e) => break Ok((&input[index..], &input[..index])),
            Ok((_rest, c)) => {
                if f(c) {
                    index += c.len_utf8();
                } else {
                    break Ok((&input[index..], &input[..index]));
                }
            }
        }
    }
}

fn skip_ws(input: &str) -> Result<(&str, ()), ParseError<'_>> {
    let (rest, _) = take_while(|c| c.is_whitespace(), input)?;
    Ok((rest, ()))
}

// f64 = -? digit* (. digit*)? ([eE] -? digit*)?


fn parse_f64(input: &str) -> Result<(&str, f64), ParseError> {
    let (restx, pos_or_neg) = match satisfy(|c| c == '-', input) {
        Ok((rest, _)) => (rest, -1.0f64),
        Err(_) => (input, 1.0f64),
    };

    let (resty, integral) = take_while(|c| c.is_digit(10), restx)?;
    let (restz, fractional) = match satisfy(|c| c == '.', resty) {
        Ok((rest, dot)) => {
            let (rest, frac) = take_while(|c| c.is_digit(10), rest)?;
            (rest, format!("{}{}", dot, frac))
        }
        Err(_) => (resty, "".to_string()),
    };

    let (restw, exponent) = match satisfy(|c| c == 'e' || c == 'E', restz) {
        Ok((rest, e)) => {
            let (rest, exp) = take_while(|c| c.is_digit(10), rest)?;
            (rest, format!("{}{}", e, exp))
        }
        Err(_) => (restz, "".to_string()),
    };

    let final_parse = format!("{}{}{}", integral, fractional, exponent);
    let (rest, s) = take_while(|c| c.is_digit(10), restw)?;
    let n = format!("{}{}", final_parse, s).parse::<f64>().map_err(|_e| ParseError::InvalidSequence(s))?;
    Ok((rest, pos_or_neg * n))
}


// fn parse_bool(input: &str) -> Result<(&str, bool), ParseError<'_>> {
//     let (rest, s) = take_while(|c| c.is_alphabetic(), input)?;
//     match s {
//         "true" => Ok((rest, true)),
//         "false" => Ok((rest, false)),
//         _ => Err(ParseError::InvalidSequence(s)),
//     }
// }

pub fn parse_expr(input: &str) -> Result<(&'_ str, Expr), ParseError<'_>> {
    let (rest, _) = skip_ws(&input)?;

    let (mut i, lhs) = parse_term(rest)?;
    let mut v = lhs;

    loop {
        let (rest, _) = skip_ws(i)?;
        let (rest, operator) = match satisfy(|c| c == '+' || c == '-', rest) {
            Ok(x) => x,
            Err(_) => break Ok((i, v)),
        };
        let (rest, rhs) = parse_term(rest)?;
        let r = rhs;

        match operator {
            '+' => v = Expr::Add(Box::from(v), Box::from(r)),
            '-' => v = Expr::Sub(Box::from(v), Box::from(r)),
            _ => unreachable!(),
        };
        i = rest;
    }
}

fn parse_term(input: &str) -> Result<(&str, Expr), ParseError<'_>> {
    let (rest, lhs) = parse_pow(input)?;
    let mut i = rest;
    let mut v = lhs;

    loop {
        let (rest, _) = skip_ws(i)?;
        let (rest, operator) = match satisfy(|c| c == '*' || c == '/', rest) {
            Ok(x) => x,
            Err(_) => break Ok((i, v)),
        };
        let (rest, rhs) = parse_pow(rest)?;
        let r = rhs;

        match operator {
            '*' => v = Expr::Mul(Box::from(v), Box::from(r)),
            '/' => v = Expr::Div(Box::from(v), Box::from(r)),
            _ => unreachable!(),
        };

        i = rest;
    }
}

fn parse_pow(input: &str) -> Result<(&str, Expr), ParseError<'_>> {
    let (rest, base) = parse_factor(input)?;
    let mut i = rest;
    let mut v = base;

    loop {
        let (rest, _) = skip_ws(i)?;
        let (rest, _) = match satisfy(|c| c == '^', rest) {
            Ok(x) => x,
            Err(_) => break Ok((i, v)),
        };
        let (rest, exp) = parse_pow(rest)?;
        let r = exp;
        v = Expr::Pow(Box::from(v), Box::from(r));
        i = rest;
    }
}

fn parse_factor(input: &str) -> Result<(&str, Expr), ParseError<'_>> {
    let (rest, _) = skip_ws(input)?;

    if let Ok((rest, num)) = parse_f64(rest) {
        return Ok((rest, Expr::Float(num)));
    }

    if let Ok((rest, name)) = parse_name(input) {
        return Ok((rest, Expr::Var(name)));
    }

    let (rest, _) = satisfy(|c| c == '(', rest)?;
    let (rest, _) = skip_ws(rest)?;
    let (rest, expr) = parse_expr(rest)?;
    let (rest, _) = skip_ws(rest)?;
    let (rest, _) = satisfy(|c| c == ')', rest)?;
    let (rest, _) = skip_ws(rest)?;

    Ok((rest, expr))
}

fn parse_name(input: &str) -> Result<(&str, String), ParseError<'_>> {
    let (rest, _) = skip_ws(input)?;
    let (rest, first_char) = satisfy(|c| c.is_ascii_alphabetic() || c == '_', rest)?;
    let (rest, _) = skip_ws(rest)?;
    let (rest, name_chars) = take_while(|c| c.is_ascii_alphanumeric() || c == '_', rest)?;
    let (rest, _) = skip_ws(rest)?;
    let name = format!("{}{}", first_char, name_chars);
    Ok((rest, name))
}

pub fn parse_statement(input: &str) -> Result<(&str, Statement), ParseError<'_>> {
    let (rest, _) = skip_ws(input)?;

    if let Ok((rest, name)) = parse_name(rest) {
        let (rest, _) = skip_ws(rest)?;

        if let Ok((rest, operator)) = satisfy(|c| c == '=', rest) {
            let (rest, _) = skip_ws(rest)?;
            let (rest, expr) = parse_expr(rest)?;
            let (rest, _) = skip_ws(rest)?;

            let statement = match operator {
                '=' => Statement::Assign(name, expr),
                _ => return Err(ParseError::InvalidChar(operator)),
            };
            return Ok((rest, statement));
        }
    }
    let (rest, expr) = parse_expr(rest)?;
    let (rest, _) = skip_ws(rest)?;
    Ok((rest, Statement::Expr(expr)))
}