use std::io;

/// This assignment explores different data types
/// and how functions work.

// function returns a 32-bit float
fn f(x: f32, y:f32) -> f32 { 
    2.0 * y + 2.0 * x * x
}

fn eulers_method(x_i: f32, x_f: f32, _y: f32, _h: f32) {
    let mut x: f32 = x_i;
    let mut y: f32 = _y;
    let mut temp: f32 = 0.0;

    while x < x_f {
        temp = f(x, y);

    }

    let result = f(x_i, y);
    println!("{result}");
}

fn main() {
    println!("Hello, world!");

    let mut x_final: String = String::new();    
    let mut x_init: String = String::new();
    let mut y_val: String = String::new();
    let mut h_val: String = String::new();

    //input for x final
    println!("Please enter the final value for x: ");
    io::stdin()
        .read_line(&mut x_final)
        .expect("Could not read line");
    let x_final: f32 = x_final.trim().parse().expect("Please type a number");
    
    //input for x initial
    println!("Please enter an initial value for x: ");
    io::stdin()
        .read_line(&mut x_init)
        .expect("Could not read line");
    let x_init: f32 = x_init.trim().parse().expect("Please type a number");

    //input for y value
    println!("Please enter a value for y: ");
    io::stdin()
        .read_line(&mut y_val)
        .expect("Could not read line");
    let y_val:f32 = y_val.trim().parse().expect("Please type a number!");

    //input for h value
    println!("Please enter a value for h: ");
    io::stdin()
        .read_line(&mut h_val)
        .expect("Could not read line");
    let h_val: f32 = h_val.trim().parse().expect("Please type a number");

    let result = eulers_method(x_init, x_final, y_val, h_val);
}