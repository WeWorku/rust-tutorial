use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret: u64 = rand::thread_rng().gen_range(1..=10);
    println!("Please input your guess.");
    let mut guess = String::new();
    let _n = match io::stdin().read_line(&mut guess){
        Ok(n) => n,
        Err(_) => {
            println!("Failed to read line.");
            return;
        }
    };

    let guess: u64 = match guess.trim().parse(){
        Ok(n) => n,
        Err(_) => {
            println!("Failed to parse input.");
            return;
        }
    };
    println!("Your guessed: {guess}");
    match guess.cmp(&secret){
        Ordering::Less => println!("Too small. actual={secret}"),
        Ordering::Greater => println!("Too big. actual={secret}"),
        _ => println!("You win")
    }
}
