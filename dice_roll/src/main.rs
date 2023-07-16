use rand::Rng;
use std::io;

fn main() {
    println!("What number you want on your dice ? :");
    let mut num_wanted = String::new();

    io::stdin()
        .read_line(&mut num_wanted)
        .expect("Failed to get the input :(");

    let num_wanted: i32 = match num_wanted.trim().parse() {
        Ok(num) => {
            if num >= 1 && num <= 6 {
                num
            } else {
                println!("No other input allowed than 1 - 6");
                -1
            }
        }
        Err(_) => -1,
    };

    loop {
        let num_on_dice: i32 = roll_the_dice();
        match num_on_dice {
            num_wanted => {
                println!("END of the roll => Num on dice => 6");
                break;
            }
            _ => {
                println!("The Number on the Dice : {:?}", num_on_dice);
            }
        }
    }
}

fn roll_the_dice() -> i32 {
    // Getting the random number from 1 to 6
    let secret_number = rand::thread_rng().gen_range(1..=6);
    return secret_number;
}
