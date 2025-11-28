use std::{env, process};
use dsa_in_cli::{run_bubble_sort};

fn main() {

    let mut args: Vec<String> = env::args().collect();

    args.remove(0);

    parse_args(args);

}

fn parse_args(mut args: Vec<String>) {

    if args.len() < 1 {
        println!("Invalid number of args type 'dsa' for help ");
        process::exit(1);
    }

    let algorithm_arg: &str = &args[0];

    match algorithm_arg {
        "dsa" => {
            print_usage();
            process::exit(1);
        },
        "bubble_sort" => {
            args.remove(0);
            if let Err(e) = run_bubble_sort(args) {
                eprintln!("{e}");
                process::exit(1);
            }
        },
        _ => {
            eprintln!("Invalid argument type 'dsa' for help");
        },
    }

}

fn print_usage() {
    println!("\
Usage: 
[algorithm_name] [args]

Algorithms:
bubble_sort => 
    usage:

    bubble_sort [arg1] ...

    arguments:
    -i => custom input : usage 'bubble_sort -i [3,1,50,27,89]'
    -d => defualt input : usage 'bubble_sort -d' 
    -s => shows steps it took : usage 'bubble_sort -d -s'
");
}
