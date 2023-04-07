fn main() {
    let mut counter = 0;

    let res = loop {
        counter += 1;

        if counter == 10 {
            break counter * 20;
        }
    };

    println!("Res : {res}");

    // * Nested loops & naming loops

    let mut count = 0;

    'counting_up: loop {
        println!("count : {count}");
        let mut remaining = 10;

        loop {
            println!("remaining : {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End Count : {count}");

    /*
    count : 0
    remaining : 10
    remaining : 9
    count : 1
    remaining : 10
    remaining : 9
    count : 2
    remaining : 10
    End Count : 2
    */

    // * While loop

    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    // iterating

    let a = [1, 2, 3, 4, 5];
    let mut index = 5;

    while index < 5 {
        println!("VAL {index} : {}", a[index]);
        index += 1;
    }

    // iteration using for loop
    for element in a {
        println!("the value is: {element}");
    }

    // more
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
