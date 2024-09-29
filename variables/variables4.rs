// variables4.rs
//
// Execute `rustlings hint variables4` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let mut x = 3; // x is immutable by default and needs to be mutable for it's value to be chnaged
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}
