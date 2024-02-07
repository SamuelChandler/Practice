use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess a Number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}");


    loop {
        println!("Please enter a guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to Read Line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You Guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too large!"), 
            Ordering::Equal => {
                println!("you win!!!");
                break;
            }
        }
    }
    
}
