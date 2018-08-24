use std::env;

use std::num::ParseIntError;

fn main() {
    let args: Vec<String> = env::args().collect();
    let i = &args[1];

    let o = add_one(i);

    match o {
        Ok(v) => println!("{}", v),
        Err(e) => println!("Bad Input: {}", e),
    }
}

fn add_one(number_str: &str) -> Result<i32, ParseIntError> {
    number_str.parse::<i32>().map(|n| n + 1)
}

#[test]
fn it_works() {
    assert_eq!(add_one("2").unwrap(), 3)
}