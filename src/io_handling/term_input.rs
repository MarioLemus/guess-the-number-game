// this code is declare with crate, so can be used as internal module
use crate::io_handling::send_msg;
use std::{io::stdin, process::exit};

pub fn get_valid_input() -> u32 {
    let mut input = String::new();
    let mut output = 0;
    
    stdin().read_line(&mut input).unwrap();
    
    if input.trim().len() < 1 {
        send_msg("\n*** Any field can be empty, please re-run the app to play again ***\n");
        exit(1)
    }
    match input.trim().parse::<u32>() {
        Ok(r) => output = r,
        Err(_) => {
            send_msg("\n*** Input must be a number, please re-run the app to play again ***\n")
        }
    };
    return output;
}