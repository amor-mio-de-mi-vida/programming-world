use std::io;

fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let result = -5 / 3;
    println!("-5 divided by 3 is {}", result);

    let tup = (500, 6.4, 1);

    let (a,mut b, c) = tup;
    b = 3f64;
    println!("The value of b is: {b}");


    let m = [1, 2, 3, 4, 5];
    
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a numebr");

    let element = m[index];

    println!("The value of the element at index {index} is: {element}");
}