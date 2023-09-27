use std::io;

/// This assignment explores different data types
/// and how functions work.

// function returns a 32-bit float
fn f(x: f32, y:f32) -> f32 { 
    2.0 * y + 2.0 * x * x
}

fn eulers_method(x: f32,y: f32) {
    let result = f(x, y);
    println!("{result}");
}

fn main() {
    println!("Hello, world!");
    
    let mut _x: String = String::new();
    let mut _y: String = String::new();

    println!("Please enter a value for x: ");

    io::stdin()
        .read_line(&mut _x)
        .expect("Could not read line");

    let _x: f32 = _x.trim().parse().expect("Please type a number");

    println!("Please enter a value for y: ");

    io::stdin()
        .read_line(&mut _y)
        .expect("Could not read line");

    let _y:f32 = _y.trim().parse().expect("Please type a number!");

    let result = eulers_method(_x, _y);
}