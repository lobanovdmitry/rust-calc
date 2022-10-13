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
        let mut user_input = String::new();
        stdin().read_line(&mut user_input).unwrap();
        let input = user_input.trim();
        match calc.calc(input) {
            Ok(result) => println!("{result}"),
            Err(e) => eprintln!("ERROR: {:?}", e.0),
        }
        stdout().flush().unwrap();
    }
}
