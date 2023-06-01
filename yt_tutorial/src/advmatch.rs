enum Discount {
    Percent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: i32,
}

pub(crate) fn main() {
    let flat = Discount::Flat(2);
    match flat {
        Discount::Flat(2) => println!("Flat 2"),
        Discount::Flat(amount) => println!("Discount of flat {:?}", amount),
        _ => (),
    }

    let concertTocket = Ticket {
        event: "Concert".to_owned(),
        price: 50,
    };

    match concertTocket {
        Ticket { price: 50, event } => println!("event @50 : {:?}", event),
        Ticket { price, .. } => println!("price : {:?}", price),
    }
    // here '..' means that any other field beside the price is ignored
}
