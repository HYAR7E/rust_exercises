#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Sqr(Box<Expr>),
}

#[derive(Debug)]
pub enum ParseError {
    NotANumber(String),
    NotEnoughOperands,
    TooManyArguments,
    EmptyInput,
}

#[derive(Debug)]
pub enum EvalError {
    DivisionByZero,
}

pub fn parse(input: &str) -> Result<Expr, ParseError> {
    let mut stack: Vec<Box<Expr>> = Vec::new();
    for word in input.split_ascii_whitespace() {
        let expr = match word {
            "+" | "-" | "*" | "/" => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    match word {
                        "+" => Expr::Add(a, b),
                        "-" => Expr::Sub(a, b),
                        "*" => Expr::Mul(a, b),
                        "/" => Expr::Div(a, b),
                        _ => unreachable!(),
                    }
                } else {
                    return Err(ParseError::NotEnoughOperands);
                }
            }
            "sqr" => {
                let a = stack.pop().ok_or(ParseError::NotEnoughOperands)?;
                Expr::Sqr(a)
            }
            _ => {
                let number = word
                    .parse()
                    .map_err(|_| ParseError::NotANumber(word.to_string()))?;
                Expr::Number(number)
            }
        };
        stack.push(Box::new(expr));
    }
    match stack.pop() {
        Some(expr) if stack.is_empty() => Ok(*expr),
        Some(_) => Err(ParseError::TooManyArguments),
        None => Err(ParseError::EmptyInput),
    }
}

pub fn eval(expr: &Expr) -> Result<i64, EvalError> {
    match expr {
        Expr::Number(number) => Ok(*number),
        Expr::Add(a, b) => Ok(eval(a)? + eval(b)?),
        Expr::Sub(a, b) => Ok(eval(a)? - eval(b)?),
        Expr::Mul(a, b) => Ok(eval(a)? * eval(b)?),
        Expr::Div(a, b) => {
            let a = eval(a)?;
            let b = eval(b)?;
            if b == 0 {
                Err(EvalError::DivisionByZero)
            } else {
                Ok(a / b)
            }
        }
        Expr::Sqr(a) => {
            let a = eval(a)?;
            Ok(a * a)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers() {
        let input = "92";
        let expr = parse(input).unwrap();
        let value = eval(&expr).unwrap();
        assert_eq!(value, 92);
    }

    #[test]
    fn not_a_number() {
        let input = "X";
        let error = parse(input).unwrap_err();
        assert!(matches!(error, ParseError::NotANumber(s) if s == "X"));
    }

    #[test]
    fn add() {
        let input = "40 2 +";
        let expr = parse(input).unwrap();
        let value = eval(&expr).unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn not_enough_operands() {
        let input = "1 +";
        let error = parse(input).unwrap_err();
        assert!(matches!(error, ParseError::NotEnoughOperands));
    }

    #[test]
    fn sub() {
        let input = "40 2 -";
        let expr = parse(input).unwrap();
        let value = eval(&expr).unwrap();
        assert_eq!(value, 38);
    }

    #[test]
    fn mul() {
        let input = "40 2 *";
        let expr = parse(input).unwrap();
        let value = eval(&expr).unwrap();
        assert_eq!(value, 80);
    }

    #[test]
    fn div() {
        let input = "40 2 /";
        let expr = parse(input).unwrap();
        let value = eval(&expr).unwrap();
        assert_eq!(value, 20);
    }

    #[test]
    fn sqr() {
        let input = "2 sqr";
        let expr = parse(input).unwrap();
        let value = eval(&expr).unwrap();
        assert_eq!(value, 4);
    }

    #[test]
    fn too_many_arguments() {
        let input = "1 1 1 +";
        let error = parse(input).unwrap_err();
        assert!(matches!(error, ParseError::TooManyArguments));
    }

    #[test]
    fn empty() {
        let input = "";
        let error = parse(input).unwrap_err();
        assert!(matches!(error, ParseError::EmptyInput));
    }

    #[test]
    fn smoke_test() {
        let input = "3 sqr 4 sqr + 5 sqr -";
        let expr = parse(input).unwrap();
        let value = eval(&expr).unwrap();
        assert_eq!(value, 0);
    }
}
