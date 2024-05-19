//Create a guess function to be able to create a random number and get the user input
use std::io;
use rand::Rng;

pub fn generate_random_number() -> u32 {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    secret_number
}

pub fn guess() {
    println!("Guess the number!");
    let secret_number = generate_random_number();
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}