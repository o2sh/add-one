extern crate num_bigint;
extern crate num_traits;

use num_bigint::{BigInt, ParseBigIntError};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("No input");
        return;
    };

    let input_str = &args[1];
    let output_str = add_one(input_str);

    match output_str {
        Ok(v) => println!("{}", v),
        Err(e) => println!("Bad Input: {}", e),
    }
}

fn add_one(number_str: &str) -> Result<BigInt, ParseBigIntError> {
    number_str.parse::<BigInt>().map(|n| n + 1)
}

#[cfg(test)]
mod tests {
    use num_traits::FromPrimitive;
    use super::*;  
    #[test] 
    fn it_works() {
        assert_eq!(add_one("2").unwrap(), BigInt::from_i64(3).unwrap());
        assert_eq!(add_one("-11").unwrap(), BigInt::from_i64(-10).unwrap());
        assert_eq!(
            add_one("1256146513513224524524524524522452165841613615616516516").unwrap(),
            "1256146513513224524524524524522452165841613615616516517"
                .parse::<BigInt>()
                .unwrap()
        );
    }
}
