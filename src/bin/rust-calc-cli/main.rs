use std::io::{stdin, stdout, Write};

use rust_calc::Calc;

fn main() {
    println!("##################################");
    println!("###           CALC           #####");
    println!("##################################");
    let calc = rust_calc::new();
    loop {
        print!(">>> ");
        stdout().flush().unwrap();
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        let user_input = buffer.trim();
        if !user_input.is_empty() {
            match calc.calc(user_input) {
                Ok(result) => println!("{result}"),
                Err(err) => println!("Error: {:?}", err.msg),
            }
        }
    }
}
