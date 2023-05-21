pub(crate) fn main() {
    let mut my_numbers: Vec<i32> = Vec::new();
    my_numbers.push(0);
    my_numbers.push(1);
    my_numbers.push(2);

    for i in &my_numbers {
        println!("{:?}", i);
    }

    let mut i: i32 = 3;
    loop {
        my_numbers.push(i);
        i += 1;
        if i == 10 {
            break;
        }
    }
    println!("After update :");
    for i in &my_numbers {
        println!("{:?}", i);
    }

    // EXERCISE
    println!("EXERCISE");

    let mut test_vec: Vec<i32> = Vec::new();
    let mut j = 10;
    loop {
        test_vec.push(j);
        j += 10;
        if j == 50 {
            break;
        }
    }

    for num in &test_vec {
        match num {
            30 => println!("thirty"),
            _ => println!("{:?}", num),
        };
    }

    println!("Length of the vector [test_vec] : {:?}", test_vec.len());
}
