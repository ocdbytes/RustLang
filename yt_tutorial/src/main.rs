pub mod r#impl;
pub mod main2;
pub mod ownership;
pub mod ruststrings;
pub mod rustvectors;

fn main() {
    let f_name: String = String::from("Arun");
    let l_name: String = String::from("Jangra");
    display_name(f_name, l_name);

    // -------------------------------------> functions
    println!("-------------------------------------> functions");
    let ans = add_num(2, 5);
    display_res(ans);

    // -------------------------------------> if else
    println!("-------------------------------------> if else");
    let decide: bool = true;
    if decide == true {
        println!("Hello !!!");
    } else {
        println!("Goodbye");
    }

    // -------------------------------------> if else if
    println!("-------------------------------------> if else if");
    let num: i32 = 5;

    if num < 5 {
        println!("<5");
    } else if num > 5 {
        println!(">5");
    } else {
        println!("=5");
    }

    // -------------------------------------> match
    println!("-------------------------------------> match");
    let b: bool = false;

    match b {
        true => println!("It is the truth"),
        false => println!("It is not the truth"),
    };

    match num {
        1 => println!("one"),
        5 => println!("woohoo :) -> 5"),
        _ => println!("blyat"),
    };

    // -------------------------------------> loop
    println!("-------------------------------------> loop");
    let mut i: i32 = 0;

    loop {
        println!("hello walt !!");
        i += 1;
        if i >= 10 {
            break;
        }
    }

    // -------------------------------------> while
    println!("-------------------------------------> while");

    let mut j: i32 = 1;
    while j < 10 {
        println!("{:?}", j);
        j += 1;
    }

    // -------------------------------------> enum
    println!("-------------------------------------> enum");

    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }
    fn which_way(go: Direction) {
        match go {
            Direction::Up => "up",
            Direction::Down => "down",
            Direction::Left => "left",
            Direction::Right => "right",
        };
    }
    which_way(Direction::Up);

    enum Color {
        Red,
        Blue,
        Green,
    }

    fn print_color(color: Color) {
        match color {
            Color::Red => println!("The color is Red"),
            Color::Blue => println!("The color is Blue"),
            Color::Green => println!("The color is Green"),
        };
    }
    print_color(Color::Red);

    // -------------------------------------> struct
    println!("-------------------------------------> struct");

    struct GroceryItem {
        stock: i32,
        price: f64,
    }

    let cereal = GroceryItem {
        stock: 10,
        price: 2.99,
    };

    println!("stuck: {:?}", cereal.stock);
    println!("price: {:?}", cereal.price);

    enum Flavour {
        Orange,
        Strawberry,
        Mango,
    }

    struct DrinkStruct {
        flavour: Flavour,
        ounce_info: i32,
    }

    let drink1 = DrinkStruct {
        flavour: Flavour::Orange,
        ounce_info: 12,
    };

    match drink1.flavour {
        Flavour::Orange => println!("Orange"),
        Flavour::Strawberry => println!("Strawberry"),
        Flavour::Mango => println!("Mango"),
    }

    println!("oz: {:?}", drink1.ounce_info);

    main2::tut2();
    // -------------------------------------> ownership
    println!("-------------------------------------> ownership");
    ownership::ownership_demo();
    // -------------------------------------> impl
    println!("-------------------------------------> impl");
    r#impl::main();
    // -------------------------------------> vectors
    println!("-------------------------------------> vectors");
    rustvectors::main();
    // -------------------------------------> strings
    println!("-------------------------------------> strings");
    ruststrings::main();
}

fn display_name(f_name: String, l_name: String) {
    println!("{} {}", f_name, l_name);
}

fn add_num(a: i32, b: i32) -> i32 {
    a + b
}

fn display_res(result: i32) {
    println!("{:?}", result);
}
