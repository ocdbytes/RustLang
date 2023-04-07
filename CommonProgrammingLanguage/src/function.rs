fn main() {
    // return scope
    // let mut x = (let y = 6); // ! ERROR : no return value of expression : let y = 6

    let _y = {
        let x = 2;
        x + 3
    }; // This scope returns value : 5
       // * No ERROR

    fn sum(a: u32, b: u32) -> u32 {
        a + b
    }

    let res = sum(2, 3);
    println!("sum(2,3) : {res}");
}
