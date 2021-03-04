use rand::Rng;
use std::cmp::Ordering;
use std::io;

//https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

            //The parse method on strings parses a string into some kind of number.
            //We need to tell Rust the exact number type we want by using let 
            //guess: u32. The colon (:) after guess tells Rust we’ll annotate the 
            //variable’s type (unsigned 32 bit integer). //the u32 annotation in 
            //this example program and the comparison with secret_number means that 
            //Rust will infer that secret_number should be a u32 as well. So now 
            //the comparison will be between two values of the same type!
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            


        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

    }
}