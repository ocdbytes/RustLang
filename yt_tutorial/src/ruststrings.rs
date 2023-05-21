struct Person {
    age: i32,
    name: String,
    color: String,
}

pub(crate) fn main() {
    /*
    String -> Owned
    &str -> borrowed String slice

    - Must use an owned String to store in a struct
    - Use &str when passing to a function
     */

    let owned_string = "owned string".to_owned();
    let another_owned = String::from("another owned");
    print_it(&owned_string);
    print_it(&another_owned);

    // EXERCISE

    let person_vec: Vec<Person> = vec![
        Person {
            age: 12,
            name: String::from("Rohan"),
            color: String::from("Red"),
        },
        Person {
            age: 18,
            name: String::from("Mohan"),
            color: String::from("Green"),
        },
        Person {
            age: 21,
            name: String::from("John"),
            color: String::from("Blue"),
        },
    ];

    for person in &person_vec {
        if person.age > 15 {
            print_info(&person);
        }
    }
}

fn print_info(person: &Person) {
    println!("Age : {:?}", person.age);
    println!("Name : {:?}", person.name);
    println!("Color : {:?}", person.color);
}

fn print_it(data: &str) {
    println!("{:?}", data);
}
