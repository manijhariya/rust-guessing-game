use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Secret Number is : {secret_number}");
    loop {
        println!("Please input your guess.");

        let mut guessed_number = String::new();
        io::stdin()
            .read_line(&mut guessed_number)
            .expect("Failed to read lines");
        let guess: u32 = match guessed_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Equal => {
                    println!("You Win!");
                    break;
                }
            Ordering::Greater => println!("Too Big!"),
        }
    }
}
