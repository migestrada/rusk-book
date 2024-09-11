/*
  This line is a kind of import.
  By default Rust has a set of items defined in the standard library
  This set is called PRELUDE
*/
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  println!("Guess the number"); // Normal Print
  
  let secret_number = rand::thread_rng().gen_range(0..=100);
  
  println!("This is the secret number: {}", secret_number);

  loop {
    println!("Please input your guess."); // Normal Print
  
    let mut guess = String::new();
  
    //let apples = 5; // immutable
    //let mut bannanas = 5; // mutable
  
    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read Line");
  
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("Please type a number!\n");
        continue;
      },
    };
  
    println!("Your guessed: {}", guess);
  
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      },
    }
  }
}
