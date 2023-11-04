
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
    Mod(Box<Expr>, Box<Expr>),
    Call(String, Vec<Expr>),
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

fn parse_f64(input: &str) -> Result<(&str, f64), ParseError> {
    let (restx, pos_or_neg) = match satisfy(|c| c == '-', input) {
        Ok((rest, _)) => (rest, -1.0f64),
        Err(_) => (input, 1.0f64),
    };

    let (resty, integral) = take_while(|c| c.is_digit(10), restx)?;
    let (restz, fractional) = match satisfy(|c| c == '.', resty) {
        Ok((rest, _)) => {
            let (rest, frac) = take_while(|c| c.is_digit(10), rest)?;
            (rest, frac)
        }
        Err(_) => (resty, "0"),
    };

    let (restw,exponent_sign,  exponent) = match satisfy(|c| c == 'e' || c == 'E', restz) {
        Ok((rest, _)) => {
            let (rest,sign, exp) = match satisfy(|c| c == '-' , rest) {
                Ok((rest, _)) => {
                    let (rest, exp) = take_while(|c| c.is_digit(10), rest)?;
                    (rest, "-", exp)
                }
                Err(_) => {
                    let (rest, exp) = take_while(|c| c.is_digit(10), rest)?;
                    (rest,"", exp)
                }
            };
            (rest,sign, exp)
        }
        Err(_) => (restz,"", "0"),
    };

    let final_parse = format!("{integral}.{fractional}e{exponent_sign}{exponent}");
    let n = final_parse.parse::<f64>().map_err(|_e| ParseError::InvalidSequence(restw))?;
    Ok((restw, pos_or_neg * n))
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
        let (rest, operator) = match satisfy(|c| c == '*' || c == '/' || c == '%', rest) {
            Ok(x) => x,
            Err(_) => break Ok((i, v)),
        };
        let (rest, rhs) = parse_pow(rest)?;
        let r = rhs;

        match operator {
            '*' => v = Expr::Mul(Box::from(v), Box::from(r)),
            '/' => v = Expr::Div(Box::from(v), Box::from(r)),
            '%' => v = Expr::Mod(Box::from(v), Box::from(r)),
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


    if let Ok((rest, name)) = parse_name(input) {
        return parse_maybe_call(name, rest);
    }

    if let Ok((rest, num)) = parse_f64(rest) {
        return Ok((rest, Expr::Float(num)));
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
    let name = format!("{}{}", first_char, name_chars).to_lowercase();
    Ok((rest, name))
}


fn parse_maybe_call(name: String, input: &str) -> Result<(&str, Expr), ParseError<'_>> {
    let (rest, e) = match satisfy(|c| c == '(', input) {
        Ok((expr, _)) => {
            let mut args = Vec::new();

            let (mut rest, exp) = parse_expr(expr)?;
            args.push(exp);

            while let Ok((new_rest, _)) = satisfy(|c| c == ',', rest) {
                let (next_rest, exp) = parse_expr(new_rest)?;
                args.push(exp);
                rest = next_rest;
            }

            let (rest, _) = satisfy(|c| c == ')', rest)?;

            (rest, Expr::Call(name,  args))
        }
        _ => {
            (input, Expr::Var(name))
        }
    };
    Ok((rest, e))
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

/*

()
^
* / %
+ -

*/