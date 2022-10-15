use crate::{Calc, CalcErr};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref OPERATIONS: HashMap<&'static str, u8> =
        HashMap::from([("+", 0), ("-", 0), ("*", 1), ("/", 1), ("^", 2)]);
    static ref NUMBER_REGEX: Regex = Regex::new("^(-?[1-9]\\d*|-?0)(\\.\\d*)?").unwrap();
}
const SIN: &str = "sin";
const COS: &str = "cos";
const TAN: &str = "tan";

pub struct SimpleCalc;

impl Calc for SimpleCalc {
    fn calc(&self, input: &str) -> Result<f64, CalcErr> {
        let rpn = convert_to_rpn(input)?;
        let number = calc_from_rpn(rpn)?;
        Ok(number)
    }
}

enum CalcExprItem {
    UniOp(&'static str, Box<dyn Fn(f64) -> f64>),
    BinOp(&'static str, Box<dyn Fn(f64, f64) -> f64>),
    Number(f64),
    OpenBracket,
    CloseBracket,
}

use CalcExprItem::*;

impl PartialEq for CalcExprItem {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (UniOp(x, _), UniOp(y, _)) => x == y,
            (BinOp(x, _), BinOp(y, _)) => x == y,
            (Number(x), Number(y)) => x == y,
            (OpenBracket, OpenBracket) => true,
            (CloseBracket, CloseBracket) => true,
            _ => false,
        }
    }
}

impl CalcErr {
    fn new(text: &str) -> CalcErr {
        Self { msg: text.to_string() }
    }
}

type CalcStack = Vec<CalcExprItem>;

/// Converts input string into vector of expressions in Reverse Polish Notation.
/// For example:
///     input: 1+2*3
///     result: vec[Number(1), Number(2), Number(3), BinOp(*), BinOp(+)]
fn convert_to_rpn(input: &str) -> Result<CalcStack, CalcErr> {
    if !input.is_ascii() {
        return Err(CalcErr::new("Input must contain only ascii characters!"));
    }
    let mut stack: CalcStack = vec![];
    let mut result: CalcStack = vec![];
    let mut i = 0;
    while i < input.len() {
        let next_char = &input[i..i + 1];
        let len = parse_prefix(next_char, &mut stack, &mut result)?;
        if len > 0 {
            i += len;
            continue;
        }
        let prefix = &input[i..];
        let len = parse_number(prefix, &mut result)?;
        if len > 0 {
            i += len;
            continue;
        }
        let len = parse_binary_op(next_char, &mut stack, &mut result)?;
        if len > 0 {
            i += len;
            continue;
        }
        let len = parse_unary_op(prefix, &mut stack)?;
        if len > 0 {
            i += len;
            continue;
        }
        return Err(CalcErr::new(&format!("Can't process at {i}")));
    }
    while let Some(ch) = stack.pop() {
        if ch == CloseBracket || ch == OpenBracket {
            return Err(CalcErr::new("Brackets don't match in the expression."));
        }
        result.push(ch);
    }
    Ok(result)
}

fn parse_prefix(next_char: &str, stack: &mut CalcStack, result: &mut CalcStack) -> Result<usize, CalcErr> {
    match next_char {
        " " | "\t" => Ok(1),
        "(" => {
            stack.push(OpenBracket);
            Ok(1)
        }
        ")" => {
            loop {
                match stack.pop() {
                    Some(OpenBracket) => break,
                    Some(x) => result.push(x),
                    None => return Err(CalcErr::new("brackets are not alignt in the expression!!!")),
                }
            }
            Ok(1)
        }
        _ => Ok(0),
    }
}

fn parse_number(input: &str, result: &mut CalcStack) -> Result<usize, CalcErr> {
    if let Some(m) = NUMBER_REGEX.find(input) {
        let num_str = &input[0..m.end()];
        let number = num_str
            .parse::<f64>()
            .map_err(|e| CalcErr::new(&format!("Failed to parse number: {} err={}", m.as_str(), e)))?;
        result.push(Number(number));
        Ok(num_str.len())
    } else {
        Ok(0)
    }
}

fn parse_binary_op(next_char: &str, stack: &mut CalcStack, result: &mut CalcStack) -> Result<usize, CalcErr> {
    if OPERATIONS.contains_key(&next_char) {
        loop {
            if let Some(top) = stack.last() {
                match top {
                    UniOp(_, _) => {
                        stack.pop().map(|x| result.push(x));
                    }
                    BinOp(x, _) => {
                        let top_op_priority = OPERATIONS.get(x);
                        let cur_op_priority = OPERATIONS.get(&next_char);
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
        let bin_op = match next_char {
            "+" => BinOp("+", Box::new(|x, y| x + y)),
            "-" => BinOp("-", Box::new(|x, y| x - y)),
            "*" => BinOp("*", Box::new(|x, y| x * y)),
            "/" => BinOp("/", Box::new(|x, y| x / y)),
            "^" => BinOp("^", Box::new(|x, y| x.powf(y))),
            _ => panic!("Operation is not supported {}.", next_char),
        };
        stack.push(bin_op);
        Ok(1)
    } else {
        Ok(0)
    }
}

fn parse_unary_op(input: &str, stack: &mut CalcStack) -> Result<usize, CalcErr> {
    let (item, len) = if input.starts_with(SIN) {
        let sin = |x: f64| rad2degree(x).sin();
        let sin_op = UniOp(SIN, Box::new(sin));
        (sin_op, 3)
    } else if input.starts_with(COS) {
        let cos = |x: f64| rad2degree(x).cos();
        let cos_op = UniOp(COS, Box::new(cos));
        (cos_op, 3)
    } else if input.starts_with(TAN) {
        let tan = |x: f64| rad2degree(x).tan();
        let tan_op = UniOp(TAN, Box::new(tan));
        (tan_op, 3)
    } else {
        return Ok(0);
    };
    stack.push(item);
    Ok(len)
}

/// Converts radians into degrees.
fn rad2degree(x: f64) -> f64 {
    x * std::f64::consts::PI / 180.0
}

/// Calculates final number based on Reverse Polish Notation.
/// For example:
///     input: vec[Number(1), Number(2), Number(3), BinOp(*), BinOp(+)]
///     result: Ok(7)
fn calc_from_rpn(rpn_vec: CalcStack) -> Result<f64, CalcErr> {
    assert!(!rpn_vec.is_empty(), "rpc_vec must be non empty!");
    let mut stack = Vec::<f64>::new();
    for item in rpn_vec {
        match item {
            UniOp(_, op) => {
                let arg = stack.pop().ok_or(CalcErr::new("No arg for unary operation."))?;
                stack.push(op(arg));
            }
            BinOp(_, op) => {
                let arg_2 = stack.pop().ok_or(CalcErr::new("Binary operation args invalid."))?;
                let arg_1 = stack.pop().ok_or(CalcErr::new("Binary operation args invalid."))?;
                stack.push(op(arg_1, arg_2));
            }
            Number(value) => {
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
