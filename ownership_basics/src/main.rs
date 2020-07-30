fn main() {

    // memory is requested, but then, 
    // returned to the OS when the variable
    // s goes out of scope
    let mut s = String::from("hello");


    s.push_str(", world!");

    println!("{}", s);



    // once scope ends, Rust calls strings "drop" function
    aliases();    
}

fn aliases() {
    let s1 = String::from("hello");
    let s2 = s1;

    // s1 and s2 point to the same memory location
    // deep copy would be VERY costly 

    // using s1 will give an ERR!
    // this is due to ownership rules, s2 owns
    // the array, s1 now is invalid (the value has been borrowed)

    // there ARE NO shallow copies, rather, a 'move' happens here
    // RUST will never make a deep copy either
    
    let s3 = String::from("hello");
    let s4 = s3.clone();

    println!("s3 = {}, s4 = {}", s3, s4);

    stack_copy();
}

fn stack_copy() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);   // x is deep copied to y since x is stored on the stack 
                                        // and the size is already known
}

