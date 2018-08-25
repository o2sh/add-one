extern crate num_bigint;
extern crate num_traits;
extern crate add_one;

use std::env;
use add_one::add_one;

fn main() {
    let mut args = env::args();

    if args.next().is_none() {
        eprintln!("Too few arguments. Expected program name and a single argument.");
        return;
    };

    let input_str = if let Some(arg) = args.next() {
        arg
    } else {
        eprintln!("Too few arguments. Expected a single argument.");
        return;
    };

    if args.next().is_some() {
        eprintln!("Too many arguments. Expected a single argument.");
        return;
    };

    let output_str = add_one(&input_str);

    match output_str {
        Ok(v) => println!("{}", v),
        Err(e) => eprintln!("Bad Input: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use num_traits::FromPrimitive; 
    use num_bigint::BigInt;
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

