use io_handling::{send_msg, get_valid_input};
use rand::{thread_rng, Rng};

mod io_handling;


fn main() {
    let secret_n = thread_rng().gen_range(1..=30);
    let mut lifes = 3;
    let mut input_res;
    
    loop {
        send_msg("\nWelcome to guess the number, the unique place where you will be glad to be thinking about numbers!");
        send_msg("Let me tell you, that the game is pretty straight forward the only action you performe");
        send_msg("is actually guessing the possible number from a list of sequential numbers starting from 1 - 30");
        send_msg("\nPlease only write numbers or else the app will finish its execution\n");
        send_msg("Good luck, and lets start!\n");
        send_msg("enter number:");

        input_res = get_valid_input();
        loop {
            if lifes == 1 {
                println!("You lost, your lifes are 0, the secret number was {secret_n}");
                break;
            }
            if input_res != secret_n {
                lifes -= 1;
                println!("The number you enter is wrong you have {lifes} resting lifes");
                input_res = get_valid_input();
                continue;
            }
            println!("Congrats the number you wrote: {input_res} is exactly the same as the secret number: {secret_n}");
            break;
        }
        break;
    };
}
