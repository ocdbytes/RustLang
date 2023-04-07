#[derive(Debug)]
enum option<T> {
    None,
    Some(T),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Dime => 1,
        Coin::Nickel => 2,
        Coin::Penny => {
            println!("This is a penny !!");
            3
        }
        Coin::Quarter(state) => {
            println!("State : {:?}", state);
            4
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

pub(crate) fn main() {
    let five = Some(5);
    let six = plus_one(five);

    // '_' var
    let dice_roll = 9;

    match dice_roll {
        1 => option1(),
        2 => option2(),
        _ => option3(),
    }

    fn option1() {}
    fn option2() {}
    fn option3() {}

    // ? Concise Control Flow
    // let mut count = 0;

    // if let Coin::Quarter(state) = coin {
    //     println!("State Quarter from {:?}", state);
    // } else {
    //     count += 1;
    // }
}
