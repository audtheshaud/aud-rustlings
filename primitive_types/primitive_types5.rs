// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age) = cat; // Set the data stored in the tuple to variable names
    // in order for the tuple data to print

    println!("{} is {} years old.", name, age);
}
