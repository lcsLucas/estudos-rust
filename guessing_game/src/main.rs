use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("The secret number 🤫: {}", secret_number);

    println!("Guess the Number!\n");

    loop {

        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");
    
        println!("You guessed: {guess}");
    
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Hum, is this a number!?");
                continue;
            }
        };
    
        match guess.cmp(&secret_number) {
            Ordering::Less => {

                println!("Too Small!");

                // if (secret_number - guess > 5) {
                //     println!("Too Small!");
                // } else {
                //     println!("Almost, but still smaller!");
                // }

            },
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }

}
