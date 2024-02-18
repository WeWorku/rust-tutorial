use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret: u64 = rand::thread_rng().gen_range(1..=10);
    println!("Please input your guess.");
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line.");
    let guess: u64 = buf.trim().parse().expect("Please type a number");
    println!("Your guessed: {buf}");
    match guess.cmp(&secret){
        Ordering::Less => println!("Too small. actual={secret}"),
        Ordering::Greater => println!("Too big. actual={secret}"),
        _ => println!("You win")
    }
}
