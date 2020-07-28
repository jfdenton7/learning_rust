use std::io;
use std::cmp::Ordering;
use std::process;

fn main() {
    let x = 5;
    println!("The value of x is {}", x);

    // x = 6; // err! must be mutable
    let mut y = 3;
    println!("the value of y is {}", y);
    y = 4;
    println!("the value of y is now {}", y);
    
    const MAX_GUESS: u32 = 100_000;
    
    let mut guess = String::new();

    io::stdin()
        .read_line(& mut guess)
        .expect("Err: failed to read line");

    // shadow to replace
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => process::exit(-1),
    };

    if guess >= MAX_GUESS {
        println!("oof");
        process::exit(-1);
    }
    


}
