use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {        
    let secret_num = rand::thread_rng().gen_range(0, 101);

    println!("Guess the number!");

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new(); // should create a new string each run

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
                Ok(num) => num, // implicit return
                Err(_) => {
                    println!("Err: Please input a number");
                    continue;
                }
            };
            
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {    // expand lambda
                println!("You win!");
                break; 
            }
        }
    }
}