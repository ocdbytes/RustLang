mod derive_debig;
mod methods;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    println!("Hello, world!");

    let mut user1 = User {
        active: true,
        username: String::from("abc@1233"),
        email: String::from("arunjngra89@gmail.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("aq6433@srmist.edu.in");

    // Creating instances from other instances

    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("abc@gmail.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    // Shorthand
    let user2 = User {
        email: String::from("abc@gmail.com"),
        ..user1
    };

    // Calling main() function from methods.rs
    methods::main();
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
