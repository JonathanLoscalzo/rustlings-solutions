// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
// No hints.

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    // if my_option.is_none() {
    //     my_option.unwrap();
    // }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    let mut my_empty_vec2 = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    my_empty_vec2.clear();

    println!("This Vec is empty, see? {:?}", my_empty_vec);
    println!("This Vec is empty, see? {:?}", my_empty_vec2);

    let mut a = 45;
    let mut b = 66;
    // Let's swap these two!
    // value_a = value_b;
    // value_b = value_a;
    std::mem::swap(&mut a, &mut b);
    println!("value a: {}; value b: {}", a, b);
}
