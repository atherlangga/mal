extern crate readline;

use std::str;
use std::ffi::CString;

use readline::readline;

fn read(input: &str) -> &str { 
    return input;
}

fn eval(ast: &str) -> &str {
    return ast;
}

fn print(output: &str) -> &str {
    return output;
}

fn rep(input: &str) -> &str {
    return print(eval(read(input)));
}

fn main() {
    let prompt = CString::new("user> ").unwrap();
    while let Ok(s) = readline(&prompt) {
        let input = str::from_utf8(s.to_bytes()).unwrap();
        println!("{}", rep(input));
    }
    println!("");
}



