use std::{env};

fn main() {

    let mut args: Vec<String> = env::args().collect();

    args.remove(0);

    parse_args(args);

}

fn parse_args(args: Vec<String>) {
    println!("{:?}", args);
}
