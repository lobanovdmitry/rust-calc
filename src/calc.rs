use crate::{Calc, CalcErr};
use regex::Regex;
use std::collections::HashMap;

pub struct SimpleCalc {
    pub conf: CalcConf,
}

impl Calc for SimpleCalc {
    fn calc(&self, input: &str) -> Result<f64, CalcErr> {
        let result = self.convert_to_rpn(input)?;
        self.calc_from_rpn(result)
    }
}

pub struct CalcConf {
    operations: HashMap<&'static str, u8>, // operation with priorities
}

impl CalcConf {
    pub fn new() -> Self {
        Self {
            operations: HashMap::from([("+", 0), ("-", 0), ("*", 1), ("/", 1), ("^", 2)]),
        }
    }
}

enum CalcExprItem {
    UniOp(&'static str, Box<dyn Fn(f64) -> f64>),
    BinOp(&'static str, Box<dyn Fn(f64, f64) -> f64>),
    Number(f64),
    OpenBracket,
    CloseBracket,
}

impl PartialEq for CalcExprItem {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::UniOp(x, _), Self::UniOp(y, _)) => x == y,
            (Self::BinOp(x, _), Self::BinOp(y, _)) => x == y,
            (Self::Number(x), Self::Number(y)) => x == y,
            (Self::OpenBracket, Self::OpenBracket) => true,
            (Self::CloseBracket, Self::CloseBracket) => true,
            _ => false,
        }
    }
}

impl CalcErr {
    fn new(text: &str) -> CalcErr {
        Self(text.to_string())
    }
}

type CalcStack = Vec<CalcExprItem>;

impl SimpleCalc {
    /// Converts input string into vection of expression in reverse polish notation.
    fn convert_to_rpn(&self, input: &str) -> Result<CalcStack, CalcErr> {
        if !input.is_ascii() {
            return Err(CalcErr::new("Input must contain only ascii characters!"));
        }
        let mut result: CalcStack = vec![];
        let mut stack: CalcStack = vec![];
        let number_regex = Regex::new("^(-?[1-9]\\d*|-?0)(\\.\\d*)?").unwrap();
        let mut i = 0;
        while i < input.len() {
            let first_char = &input[i..i + 1];
            if first_char == " " || first_char == "\t" {
                // ignoring whitespaces and tabs
                i += 1;
                continue;
            }
            if first_char == "(" {
                stack.push(CalcExprItem::OpenBracket);
                i += 1;
                continue;
            }
            if first_char == ")" {
                loop {
                    match stack.pop() {
                        Some(CalcExprItem::OpenBracket) => break,
                        Some(x) => result.push(x),
                        None => return Err(CalcErr::new("brackets are not alignt in the expression!!!")),
                    }
                }
                i += 1;
                continue;
            }
            let prefix = &input[i..];
            // processing of number
            match number_regex.find(prefix) {
                Some(m) if m.start() == 0 => {
                    let num_str = &prefix[0..m.end()];
                    let number = num_str
                        .parse::<f64>()
                        .map_err(|e| CalcErr::new(&format!("failed to parse number: {} err={}", m.as_str(), e)))?;
                    result.push(CalcExprItem::Number(number));
                    i += num_str.len();
                    continue;
                }
                _ => {}
            }

            // processing binary infix functions
            if ["+", "-", "*", "/", "^"].contains(&first_char) {
                loop {
                    let need_push;
                    if let Some(top) = stack.last() {
                        match top {
                            CalcExprItem::UniOp(_, _) => {
                                need_push = true;
                            }
                            CalcExprItem::BinOp(x, _) => {
                                if self.conf.operations.get(x) >= self.conf.operations.get(&first_char) {
                                    need_push = true;
                                } else {
                                    break;
                                }
                            }
                            _ => break,
                        }
                    } else {
                        break;
                    }
                    if need_push {
                        let x = stack.pop().unwrap();
                        result.push(x);
                    }
                }
                match first_char {
                    "+" => stack.push(CalcExprItem::BinOp("+", Box::new(|x, y| x + y))),
                    "-" => stack.push(CalcExprItem::BinOp("-", Box::new(|x, y| x - y))),
                    "*" => stack.push(CalcExprItem::BinOp("*", Box::new(|x, y| x * y))),
                    "/" => stack.push(CalcExprItem::BinOp("/", Box::new(|x, y| x / y))),
                    "^" => stack.push(CalcExprItem::BinOp("^", Box::new(|x, y| x.powf(y)))),
                    _ => panic!("why we here then???"),
                }
                i += 1;
                continue;
            }
            // processing unary prefix functions
            if prefix.starts_with("sin") {
                stack.push(CalcExprItem::UniOp(
                    "sin",
                    Box::new(|x| f64::sin(x * std::f64::consts::PI / 180.0)),
                ));
                i += 3;
                continue;
            }
            if prefix.starts_with("cos") {
                stack.push(CalcExprItem::UniOp(
                    "cos",
                    Box::new(|x| f64::cos(x * std::f64::consts::PI / 180.0)),
                ));
                i += 3;
                continue;
            }
            if prefix.starts_with("tan") {
                stack.push(CalcExprItem::UniOp(
                    "tan",
                    Box::new(|x| f64::tan(x * std::f64::consts::PI / 180.0)),
                ));
                i += 3;
                continue;
            }
            return Err(CalcErr::new(&format!("Can't process {i}")));
        }
        while let Some(ch) = stack.pop() {
            if ch == CalcExprItem::CloseBracket || ch == CalcExprItem::OpenBracket {
                return Err(CalcErr::new("Brackets don't match in the expression."));
            }
            result.push(ch);
        }
        Ok(result)
    }

    fn calc_from_rpn(&self, expr_items: Vec<CalcExprItem>) -> Result<f64, CalcErr> {
        let mut stack = Vec::<f64>::new();
        for item in expr_items {
            match item {
                CalcExprItem::UniOp(_, op) => {
                    let arg = stack.pop().ok_or(CalcErr::new("No arg for unary operation."))?;
                    stack.push(op(arg));
                }
                CalcExprItem::BinOp(_, op) => {
                    let arg_2 = stack
                        .pop()
                        .ok_or(CalcErr::new("Illegal state: binary operation args invalid."))?;
                    let arg_1 = stack
                        .pop()
                        .ok_or(CalcErr::new("Illegal state: binary operation args invalid."))?;
                    stack.push(op(arg_1, arg_2));
                }
                CalcExprItem::Number(value) => stack.push(value),
                _ => return Err(CalcErr::new("wtf?")),
            }
        }
        let result = stack.pop().ok_or(CalcErr::new("Illegal state: stack is empty."))?;
        if !stack.is_empty() {
            return Err(CalcErr::new(&format!(
                "Illegal state: stack is not empty after compute. remainings: {:?}",
                stack
            )));
        }
        Ok(result)
    }
}
