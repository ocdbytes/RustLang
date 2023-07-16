use std::collections::HashMap;

fn main() {
    // declaring vector
    let mut v: Vec<i32> = Vec::new();
    v.push(0);
    v.push(0);

    let v1 = vec![1, 2, 3];
    let third: &i32 = &v1[2];
    let third_func_way: Option<&i32> = v1.get(2);
    match third_func_way {
        Some(_third_func_way) => println!("The third element is : {third}"),
        None => println!("There is no third element"),
    }

    // looping
    for i in &v {
        println!("{i}");
    }
    for i in &mut v {
        *i += 50;
    }

    // ! Hashmaps
    // !---------

    let mut scores: HashMap<u32, u32> = HashMap::new();
    scores.insert(12, 13);
    scores.insert(14, 15);

    for (key, value) in &scores {
        println!("{key} : {value}");
    }

    // Accessing values

    let key = 12;
    let value = scores.get(&key).copied().unwrap_or(0);

    // Inserting a value if value doesn't exist

    scores.entry(10).or_insert(100);

    // Updating the value
    let text = String::from("hello world hello coder");
    let mut map: HashMap<String, u32> = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word.to_string()).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Red");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    match score {
        0 => {
            println!("Element not found");
        }
        n => {
            println!("Element found : {:?}", n);
        }
    }
}
