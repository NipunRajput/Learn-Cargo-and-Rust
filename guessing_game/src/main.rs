use std::io;  //to obtain the user input/output
use rand::Rng;
//prelude 
fn main(){  
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=10000);
    println!("Please input your guess.");
    let mut guess=String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guessed:{}",guess);
    println!("The secret number is: {secret_number}");
}
