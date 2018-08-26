extern crate add_one;

use add_one::add_one;
use std::env;
use std::io::stdout;

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

    match add_one(input_str.as_bytes(), &mut stdout()) {
        Ok(()) => println!(),
        Err(e) => eprintln!("Error: {}", e),
    }
}
