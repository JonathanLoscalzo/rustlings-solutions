// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

fn main() {
    // https://doc.rust-lang.org/rust-by-example/std/vec.html
    let mut a: Vec<u8> = (0..100).collect();
    a.push(1);

    if a.len() >= 100 {
        println!("Updated vector: {:?}", a);
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}
