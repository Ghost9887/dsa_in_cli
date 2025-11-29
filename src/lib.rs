use std::{error::Error, thread, time};

pub fn run_bubble_sort(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut custom_input = false;
    let mut default_input = false;
    let mut show_steps = false;
    
    if args.len() < 1 {
        return Err("Invalid number of arguments".into());
    }
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "-d" => {
                default_input = true;
                custom_input = false;
            },
            "-i" => {
                custom_input = true;
                default_input = false;
                if index + 1 < args.len() && args[index + 1].starts_with('[') && args[index + 1].ends_with(']'){
                    index += 1;
                }
                else {
                     return Err("Missing the custom input argument '[...]'".into());
                }
            },
            "-s" => {
                show_steps = true;
            }
            _ => {
                return Err("Argument not valid type 'dsa' for help".into());
            }
        }
        index += 1;
    }

    let mut input;

    if custom_input {
        let user_input = &args[1];   
        input = parse_custom_input(user_input);
    }
    else if default_input{
        input = vec![30.0, 21.0, 6.0, 78.0, 92.0, 14.0, 50.0, 33.0, 68.0];
    }
    else {
        return Err("No input provided for bubble sort 'dsa' for help".into());
    }

    println!("Input: {:?}", input);
    let mut moved = true;
    let mut step = 1;
    while moved {
        if show_steps {
            println!("");
            println!("{}:", step);
            print!("");
            println!("{:?}\n", input);
            //clear screen
            //print!("\x1B[2J\x1B[1;1H");
            step += 1;
            thread::sleep(time::Duration::from_millis(500));
        }
        moved = false;
        for i in 0..input.len() - 1{
            if i + 1 > input.len() - 1 {
                break;
            }
            let a = input[i];
            let b = input[i + 1];
            if a > b {
                input[i] = b;
                input[i + 1] = a;
                moved = true;
            }            
        }
    }
    println!("Result: {:?}", input);
    Ok(())
}

pub fn run_selection_sort(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut custom_input = false;
    let mut default_input = false;
    let mut show_steps = false;
    
    if args.len() < 1 {
        return Err("Invalid number of arguments".into());
    }
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "-d" => {
                default_input = true;
                custom_input = false;
            },
            "-i" => {
                custom_input = true;
                default_input = false;
                if index + 1 < args.len() && args[index + 1].starts_with('[') && args[index + 1].ends_with(']'){
                    index += 1;
                }
                else {
                     return Err("Missing the custom input argument '[...]'".into());
                }
            },
            "-s" => {
                show_steps = true;
            }
            _ => {
                return Err("Argument not valid type 'dsa' for help".into());
            }
        }
        index += 1;
    }

    let mut input;

    if custom_input {
        let user_input = &args[1];   
        input = parse_custom_input(user_input);
    }
    else if default_input{
        input = vec![30.0, 21.0, 6.0, 78.0, 92.0, 14.0, 50.0, 33.0, 68.0];
    }
    else {
        return Err("No input provided for selection sort 'dsa' for help".into());
    }
    
    println!("Input: {:?}", input);
    
    for i in 0..input.len() {
        let mut smallest_value = input[i];
        let mut index_of_smallest_value = i;
        for j in i..input.len() {
            if input[j] < smallest_value {
                smallest_value = input[j];
                index_of_smallest_value = j;
            }
        }
        let temp = input[i];
        input[i] = input[index_of_smallest_value];
        input[index_of_smallest_value] = temp;
        
        if show_steps {
            println!("");
            println!("{}:", i + 1);
            print!("");
            println!("{:?}\n", input);
            //clear screen
            //print!("\x1B[2J\x1B[1;1H");
            thread::sleep(time::Duration::from_millis(500));
        }
    }

    println!("Result: {:?}", input);
    Ok(())   
}

fn parse_custom_input(user_input: &str) -> Vec<f32> {
    let mut input: Vec<f32> = Vec::new();
    let mut string_number = String::new();
        for c in user_input.chars() {
            if c.is_numeric() {
            string_number.push(c);
            }
            else {
                if string_number.len() > 0 {
                    let number = string_number.parse::<f32>()
                        .expect("Faile to parse input");
                    input.push(number);
                    string_number = String::new();
                }
            }
        }
    input
}
