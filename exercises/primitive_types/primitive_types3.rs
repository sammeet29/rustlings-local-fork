// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

use std::mem;

fn main() {
    let a : [i32; 103] = [4; 103];

    println!("Size of the array in bytes:{}", mem::size_of_val(&a));
    // println!("Size of the array {}", mem::size_of_val(&a)/mem::size_of_val(i32));

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}
