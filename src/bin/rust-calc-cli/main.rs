use std::io::{stdin, stdout, Write};

use rust_calc::calc::Calc;

fn main() {
    println!("Hey, this is rust calc for cli!!!");
    let calc = rust_calc::simple_calc();
    loop {
        println!("Enter expresssion: ");
        stdout().flush().unwrap();
        let mut user_input = String::new();
        stdin().read_line(&mut user_input).expect("Bad input");
        let input = user_input.trim();
        match calc.calc(input) {
            Ok(result) => println!("Result: {result}"),
            Err(e) => println!("Can't calculate: {:?}", e),
        }
        stdout().flush().unwrap();
    }
}
