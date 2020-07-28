fn main() {
    // integers follow form i{size} and u{size}

    let x = 2.0; // compiler decides type if f64
    let y: f32 = 3.0; // explicit

    let sum = 5 + 3;
    let diff = 3 - 2;
    let prod = 4 * 30;
    let quotient = 56.6 / 32.2;
    let remain = 43 % 5;

    // bools:
    let t = true;
    let f: bool = false;

    // chars
    let c = 'c'; // single quotes


    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 3);

    // deconstruct the tuple
    let (a, b, c ) = tup;

    println!("we see then: {}", b);

    // you can also grab via access operator
    let integer = tup.0;
    let float = tup.1;
    let unsign = tup.2;

    // arrays

    let arr = [1, 2, 3, 4, 5]; // arrays do NOT grow, unlike std::Vec

    let weekend = ["Saturday", "Sunday"];

    // to annotate, we have
    // goes by [type; size]
    let arr: [i32; 5] = arr;

    let arr = [3; 5]; // init value; size

    println!("arr[0] = {}", arr[0]);

    // if you acccess out of bounds, you get a runtime err!




}
