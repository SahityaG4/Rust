use std::io;

fn main() {
    println!("Welcome to guessing game, I am gudia and I will be your opponent");
    println!("Now guess a number");
    let mut your_guess = String::new();
    io::stdin()
        .read_line(&mut your_guess)
        .expect("I can't read it try properly");
    println!("your guess is: {your_guess}")
    
}
