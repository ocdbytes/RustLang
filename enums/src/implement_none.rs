#[derive(Debug)]

// Method to use None Value in Rust Lang
// -------------------------------------

enum Option<T> {
    None,
    Some(T),
}

pub(crate) fn main() {
    let some_num = Some(5);
    let some_char = Some('a');

    let none_num: Option<i32> = Option::None;
}
