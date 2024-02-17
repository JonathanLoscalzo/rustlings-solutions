// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let data = "Rust is great!".to_string();
    // get_char(data.clone());
    get_char(&data);
    string_uppercase(data);

    // let mut y: i32 = 0;
    // let z: &i32 = &32;

    // {
    //     let x = 5;
    //     y = x;
    //     println!("{}", z);
    // }

    // println!("{} {}", y, z);

    // let s = String::from("papito");
    // let s2 = s.clone();
    // println!("{} {}", s, s2);

    // let mut s = String::from("hello");
    // let mut s2 = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s2;

    // println!("{}, {}", r1, r2);
    // println!("{}, {}", r1, r2);

    let mut x = 5;
    let y = {
        let y = &mut x;
        *y = 6;
        *y
    };
    println!("{} {}", x, y);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();
    data = data + "!!!!";
    data.push_str("pepe");
    println!("{}", data);
}
