#[derive(Debug, PartialEq)]
enum Expr {
    Number(i64),
    Sum(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Times(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Sqr(Box<Expr>),
}

#[derive(Debug, PartialEq)]
enum ParseError {
    UnkownSymbol,
    NotIntegerNumber,
    WrongOrderOperator,
    OperatingNonNumbers,
    StackNonConvergent,
}

#[derive(Debug, PartialEq)]
enum EvalError {
    DivisonBy0,
    IntegerOverflow,
}

fn parse(input_expression: String) -> Result<Expr, ParseError> {
    let mut expr_stack: Vec<Expr> = Vec::new();
    for word in input_expression.split_ascii_whitespace() {
        match word {
            "+" => match (expr_stack.pop(), expr_stack.pop()) {
                (Some(second), Some(first)) => {
                    expr_stack.push(Expr::Sum(Box::new(first), Box::new(second)))
                }
                _ => return Err(ParseError::WrongOrderOperator),
            },
            "-" => match (expr_stack.pop(), expr_stack.pop()) {
                (Some(second), Some(first)) => {
                    expr_stack.push(Expr::Sub(Box::new(first), Box::new(second)))
                }
                _ => return Err(ParseError::WrongOrderOperator),
            },
            "*" => match (expr_stack.pop(), expr_stack.pop()) {
                (Some(second), Some(first)) => {
                    expr_stack.push(Expr::Times(Box::new(first), Box::new(second)))
                }
                _ => return Err(ParseError::WrongOrderOperator),
            },
            "/" => match (expr_stack.pop(), expr_stack.pop()) {
                (Some(second), Some(first)) => {
                    expr_stack.push(Expr::Div(Box::new(first), Box::new(second)))
                }
                _ => return Err(ParseError::WrongOrderOperator),
            },
            "sqrt" => match expr_stack.pop() {
                Some(first) => expr_stack.push(Expr::Sqr(Box::new(first))),
                _ => return Err(ParseError::WrongOrderOperator),
            },
            n => {
                let number = n.parse::<i64>();
                match number {
                    Ok(number) => {
                        expr_stack.push(Expr::Number(number));
                    }
                    _ => return Err(ParseError::NotIntegerNumber),
                }
            }
        }
    }
    if expr_stack.len() != 1 {
        Err(ParseError::StackNonConvergent)
    } else {
        Ok(expr_stack.pop().unwrap())
    }
}

fn eval(expr: Expr) -> Result<i64, EvalError> {
    match expr {
        Expr::Number(num) => Ok(num),
        Expr::Sum(expr1, expr2) => {
            let left = eval(*expr1)?;
            let right = eval(*expr2)?;
            match left.checked_add(right) {
                Some(result) => Ok(result),
                _ => Err(EvalError::IntegerOverflow),
            }
        }
        Expr::Sub(expr1, expr2) => {
            let left = eval(*expr1)?;
            let right = eval(*expr2)?;
            match left.checked_sub(right) {
                Some(result) => Ok(result),
                _ => Err(EvalError::IntegerOverflow),
            }
        }
        Expr::Times(expr1, expr2) => {
            let left = eval(*expr1)?;
            let right = eval(*expr2)?;
            match left.checked_mul(right) {
                Some(result) => Ok(result),
                _ => Err(EvalError::IntegerOverflow),
            }
        }
        Expr::Div(expr1, expr2) => {
            let right = eval(*expr2)?;
            if right == 0 {
                return Err(EvalError::DivisonBy0);
            }
            let left = eval(*expr1)?;
            match left.checked_div(right) {
                Some(result) => Ok(result),
                _ => Err(EvalError::IntegerOverflow),
            }
        }
        Expr::Sqr(expr1) => {
            let num = eval(*expr1)?;
            Ok(f64::sqrt(num as f64) as i64)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_num() {
        let expression = String::from("88");
        let chain_expression = Expr::Number(88);
        assert_eq!(parse(expression).unwrap(), chain_expression);
    }

    #[test]
    fn test_parse_num_fail() {
        let expression = String::from("40.123");
        assert_eq!(parse(expression), Err(ParseError::NotIntegerNumber))
    }

    #[test]
    fn test_parse_sum() {
        let expression = String::from("40 2 +");
        let chain_expression = Expr::Sum(Box::new(Expr::Number(40)), Box::new(Expr::Number(2)));
        assert_eq!(parse(expression).unwrap(), chain_expression);
    }

    #[test]
    fn test_parse_sub() {
        let expression = String::from("40 2 -");
        let chain_expression = Expr::Sub(Box::new(Expr::Number(40)), Box::new(Expr::Number(2)));
        assert_eq!(parse(expression).unwrap(), chain_expression);
    }

    #[test]
    fn test_parse_times() {
        let expression = String::from("40 2 *");
        let chain_expression = Expr::Times(Box::new(Expr::Number(40)), Box::new(Expr::Number(2)));
        assert_eq!(parse(expression).unwrap(), chain_expression);
    }

    #[test]
    fn test_parse_times_neg() {
        let expression = String::from("40 -2 *");
        let chain_expression = Expr::Times(Box::new(Expr::Number(40)), Box::new(Expr::Number(-2)));
        assert_eq!(parse(expression).unwrap(), chain_expression);
    }

    #[test]
    fn test_parse_sqrt() {
        let expression = String::from("16 sqrt");
        let chain_expression = Expr::Sqr(Box::new(Expr::Number(16)));
        assert_eq!(parse(expression).unwrap(), chain_expression);
    }

    #[test]
    fn test_eval_num() {
        let sentence = Expr::Number(32);
        assert_eq!(eval(sentence).unwrap(), 32);
    }

    #[test]
    fn test_eval_sum() {
        let left = Expr::Number(32);
        let right = Expr::Number(16);
        let sentence = Expr::Sum(Box::new(left), Box::new(right));
        assert_eq!(eval(sentence).unwrap(), 48);
    }

    #[test]
    fn test_eval_sub() {
        let left = Expr::Number(32);
        let right = Expr::Number(16);
        let sentence = Expr::Sub(Box::new(left), Box::new(right));
        assert_eq!(eval(sentence).unwrap(), 16);
    }

    #[test]
    fn test_eval_times() {
        let left = Expr::Number(32);
        let right = Expr::Number(16);
        let sentence = Expr::Times(Box::new(left), Box::new(right));
        assert_eq!(eval(sentence).unwrap(), 32 * 16);
    }

    #[test]
    fn test_eval_sqrt() {
        let left = Expr::Number(16);
        let sentence = Expr::Sqr(Box::new(left));
        assert_eq!(eval(sentence).unwrap(), 4);
    }

    #[test]
    fn smoke_test_1() {
        let expression = String::from("17 18 + 15 - 2 +");
        let expr = parse(expression).unwrap();
        let result = eval(expr).unwrap();
        assert_eq!(result, 22);
    }

    #[test]
    fn smoke_test_2() {
        let expression = String::from("3 4 * 2 - 5 +");
        let expr = parse(expression).unwrap();
        let result = eval(expr).unwrap();
        assert_eq!(result, 15);
    }

    #[test]
    fn smoke_test_3() {
        let expression = String::from("7 -4 * 18 + 15 -");
        let expr = parse(expression).unwrap();
        let result = eval(expr).unwrap();
        assert_eq!(result, -25);
    }

    #[test]
    fn smoke_test_4() {
        let expression = String::from("25 0 * 7 +");
        let expr = parse(expression).unwrap();
        let result = eval(expr).unwrap();
        assert_eq!(result, 7);
    }

    #[test]
    fn smoke_test_5() {
        let expression = String::from("25 5 / 3 * 7 + 2 -");
        let expr = parse(expression).unwrap();
        let result = eval(expr).unwrap();
        assert_eq!(result, 20);
    }

    #[test]
    fn smoke_test_6() {
        let expression = String::from("16 4 / 5 * 2 / 7 - 2 +");
        let expr = parse(expression).unwrap();
        let result = eval(expr).unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    fn smoke_test_7() {
        let expression = String::from("16 4 * sqrt 2 +");
        let expr = parse(expression).unwrap();
        let result = eval(expr).unwrap();
        assert_eq!(result, 10);
    }

    #[test]
    fn test_empty_expression() {
        let expression = String::from("");
        let expr = parse(expression);
        assert_eq!(expr, Err(ParseError::StackNonConvergent));
    }

    #[test]
    fn test_division_by_0() {
        let expression = String::from("25 0 /");
        let expr = parse(expression).unwrap();
        let result = eval(expr);
        assert_eq!(result, Err(EvalError::DivisonBy0));
    }

    #[test]
    fn test_integer_overflow() {
        let expression = String::from("123456789 123456789 * 1234 *");
        let expr = parse(expression).unwrap();
        let result = eval(expr);
        assert_eq!(result, Err(EvalError::IntegerOverflow));
    }

    #[test]
    fn smoke_test_0() {
        // FIXME: does question mark doesn't work on non-function expressions?
        // let res: Result<(), EvalError> = {
        //     let expression = String::from("17 18 +");
        //     let expr = parse(expression)?;
        //     let result = eval(expr)?;
        //     assert_eq!(result, 35);
        //     Ok(())
        // };
        // match res {
        //     Ok(_) => {}
        //     Err(_) => {}
        // }
    }
}
