
fn main() {
    let x = 5;
    let mut y = 5;
    
    match x {
        // the `r` inside the match has the type `&i32`
        ref r => println!("Got a reference to {}", r),
    }
    
    match y {
        // the `mr` inside the match has the type `&i32` and is mutable
        ref mut mr => println!("Got a mutable reference to {}", mr),
    }
}