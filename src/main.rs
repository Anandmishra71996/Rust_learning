use std::ops::Add;

fn main() {
    let mut val = 6;
    add_two(val);
    println!("{}", val); // we can pass the val variable because i32 implement copy traits which does clone of scaler variables
}
fn add_two(mut val: i32) {
    val += 2;
}
// fn add_two(val: &mut i8) -> i8 {}
