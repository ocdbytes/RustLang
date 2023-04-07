fn main() {
    // ! [ERROR : If we are modifying the variable -> It need to be 'mut' ] let x = 5;
    let mut x = 5;
    println!("x : {x}");
    x = 7;
    println!("x : {x}");

    // * shadowing

    let x = 10;
    let x = x + 1;

    {
        let x = x * 2;
        println!("Inner scope [x] : {x}");
    }

    println!("Outer scope [x] : {x}");

    // * changing types

    let space = "    "; // string
    let space = space.len(); // number

    // * No error : It saves space in memory

    // * DATA TYPES
    // * ----------

    // Number
    /*
        unsigned | signed
        u8 : i8
        u16 : i16
        .
        .
        .
    */

    // Tuple

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("[x] : {x}, [y] : {y}, [z] : {z}");

    let five_hundred = tup.0;
    println!("[tup.0] : {five_hundred}");

    // Array

    let a = [1, 2, 3, 4, 5]; // dynamic size
    let b: [i32; 5] = [1, 2, 3, 4, 5]; // defined size
    let c = [3; 5]; // [3,3,3,3,3]
    let y = a[0];
    println!("first of a : {y}");
}
