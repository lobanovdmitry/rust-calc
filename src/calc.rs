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
    number_regex: Regex,
}

impl CalcConf {
    pub fn new() -> Self {
        Self {
            operations: HashMap::from([("+", 0), ("-", 0), ("*", 1), ("/", 1), ("^", 2)]),
            number_regex: Regex::new("^(-?[1-9]\\d*|-?0)(\\.\\d*)?").unwrap(),
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
    /// Converts input string into vection of expression in Reverse Polish Notation.
    fn convert_to_rpn(&self, input: &str) -> Result<CalcStack, CalcErr> {
        if !input.is_ascii() {
            return Err(CalcErr::new("Input must contain only ascii characters!"));
        }
        let mut stack: CalcStack = vec![];
        let mut result: CalcStack = vec![];
        let mut i = 0;
        while i < input.len() {
            let next_char = &input[i..i + 1];
            let len = self.parse_prefix(next_char, &mut stack, &mut result)?;
            if len > 0 {
                i += len;
                continue;
            }
            let prefix = &input[i..];
            let len = self.parse_number(prefix, &mut result)?;
            if len > 0 {
                i += len;
                continue;
            }
            let len = self.parse_binary_op(next_char, &mut stack, &mut result)?;
            if len > 0 {
                i += len;
                continue;
            }
            let len = self.parse_unary_op(prefix, &mut stack)?;
            if len > 0 {
                i += len;
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

    fn parse_prefix(&self, next_char: &str, stack: &mut CalcStack, result: &mut CalcStack) -> Result<usize, CalcErr> {
        match next_char {
            " " | "\t" => Ok(1),
            "(" => {
                stack.push(CalcExprItem::OpenBracket);
                Ok(1)
            }
            ")" => {
                loop {
                    match stack.pop() {
                        Some(CalcExprItem::OpenBracket) => break,
                        Some(x) => result.push(x),
                        None => return Err(CalcErr::new("brackets are not alignt in the expression!!!")),
                    }
                }
                Ok(1)
            }
            _ => Ok(0),
        }
    }

    fn parse_number(&self, input: &str, result: &mut CalcStack) -> Result<usize, CalcErr> {
        if let Some(m) = self.conf.number_regex.find(input) {
            let num_str = &input[0..m.end()];
            let number = num_str
                .parse::<f64>()
                .map_err(|e| CalcErr::new(&format!("Failed to parse number: {} err={}", m.as_str(), e)))?;
            result.push(CalcExprItem::Number(number));
            Ok(num_str.len())
        } else {
            Ok(0)
        }
    }

    fn parse_binary_op(
        &self,
        next_char: &str,
        stack: &mut CalcStack,
        result: &mut CalcStack,
    ) -> Result<usize, CalcErr> {
        if ["+", "-", "*", "/", "^"].contains(&next_char) {
            loop {
                if let Some(top) = stack.last() {
                    match top {
                        CalcExprItem::UniOp(_, _) => {
                            stack.pop().map(|x| result.push(x));
                        }
                        CalcExprItem::BinOp(x, _) => {
                            let top_op_priority = self.conf.operations.get(x);
                            let cur_op_priority = self.conf.operations.get(&next_char);
                            if top_op_priority >= cur_op_priority {
                                stack.pop().map(|x| result.push(x));
                            } else {
                                break;
                            }
                        }
                        _ => break,
                    }
                } else {
                    break;
                }
            }
            match next_char {
                "+" => stack.push(CalcExprItem::BinOp("+", Box::new(|x, y| x + y))),
                "-" => stack.push(CalcExprItem::BinOp("-", Box::new(|x, y| x - y))),
                "*" => stack.push(CalcExprItem::BinOp("*", Box::new(|x, y| x * y))),
                "/" => stack.push(CalcExprItem::BinOp("/", Box::new(|x, y| x / y))),
                "^" => stack.push(CalcExprItem::BinOp("^", Box::new(|x, y| x.powf(y)))),
                _ => panic!("why we here then???"),
            }
            Ok(1)
        } else {
            Ok(0)
        }
    }

    fn parse_unary_op(&self, input: &str, stack: &mut CalcStack) -> Result<usize, CalcErr> {
        const SIN: &str = "sin";
        const COS: &str = "cos";
        const TAN: &str = "tan";
        let (item, len) = if input.starts_with(SIN) {
            let sin = |x: f64| SimpleCalc::rad2degree(x).sin();
            let sin_op = CalcExprItem::UniOp(SIN, Box::new(sin));
            (sin_op, 3)
        } else if input.starts_with(COS) {
            let cos = |x: f64| SimpleCalc::rad2degree(x).cos();
            let cos_op = CalcExprItem::UniOp(COS, Box::new(cos));
            (cos_op, 3)
        } else if input.starts_with(TAN) {
            let tan = |x: f64| SimpleCalc::rad2degree(x).tan();
            let tan_op = CalcExprItem::UniOp(TAN, Box::new(tan));
            (tan_op, 3)
        } else {
            return Ok(0);
        };
        stack.push(item);
        Ok(len)
    }

    fn rad2degree(x: f64) -> f64 {
        x * std::f64::consts::PI / 180.0
    }

    /// Calculates final number based on Reverse Polish Notation.
    fn calc_from_rpn(&self, rpn_vec: CalcStack) -> Result<f64, CalcErr> {
        assert!(!rpn_vec.is_empty(), "rpc_vec must be non empty!");
        let mut stack = Vec::<f64>::new();
        for item in rpn_vec {
            match item {
                CalcExprItem::UniOp(_, op) => {
                    let arg = stack.pop().ok_or(CalcErr::new("No arg for unary operation."))?;
                    stack.push(op(arg));
                }
                CalcExprItem::BinOp(_, op) => {
                    let arg_2 = stack.pop().ok_or(CalcErr::new("Binary operation args invalid."))?;
                    let arg_1 = stack.pop().ok_or(CalcErr::new("Binary operation args invalid."))?;
                    stack.push(op(arg_1, arg_2));
                }
                CalcExprItem::Number(value) => {
                    stack.push(value);
                }
                _ => {
                    panic!("RPN must contain only unary or binary operation or number.");
                }
            }
        }
        let result = stack.pop().ok_or(CalcErr::new("Stack is empty."))?;
        if !stack.is_empty() {
            panic!("Stack is not empty after calc. Remainings: {:?}", stack);
        }
        Ok(result)
    }
}
