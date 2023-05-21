pub(crate) fn tut2() {
    // -------------------------------------> tuples
    println!("-------------------------------------> tuple");

    let cord = (2, 3);
    println!("{:?} {:?}", cord.0, cord.1);

    let (x, y) = get_cord();

    if y > 5 {
        println!("y is >5");
    } else if y < 5 {
        println!("y is <5");
    } else {
        println!("y is =5");
    }

    // -------------------------------------> expressions
    println!("-------------------------------------> expressions");

    let my_num = 3;
    let is_lt_5 = my_num < 5;

    let message: &str = match my_num {
        1 => "hello",
        _ => "goodbye",
    };
    println!("Message : {:?}", message);
    println!("Is my_num < 5 : {:?}", is_lt_5);

    enum Access {
        Admin,
        Manager,
        User,
        Guest,
    }

    let access_level = Access::Guest;
    let can_access_file = match access_level {
        Access::Admin => true,
        Access::Manager => true,
        Access::User => false,
        Access::Guest => false,
    };

    println!("File Access Status : {:?}", can_access_file);
}

fn get_cord() -> (i32, i32) {
    (10, 57)
}
