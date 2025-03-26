use std::io;
use std::io::*;
fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");

    let mut my_int: i32 = input
        .trim()
        .parse()
        .expect("please give me correct string number!");

    println!("{}", my_int);

    while my_int != 0 {
        print!("\n{my_int} ");
        if my_int % 3 == 0 {
            print!("Fizz");
        }
        if my_int % 5 == 0 {
            print!("Buzz");
        }
        my_int -= 1;
    }
}
