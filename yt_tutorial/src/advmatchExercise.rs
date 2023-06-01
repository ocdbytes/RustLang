enum Ticket {
    Backstage(f64, String),
    VIP(f64, String),
    Standard(f64),
}

pub(crate) fn main() {
    let ticketsVec: Vec<Ticket> = vec![
        Ticket::Backstage(30.50, "John".to_owned()),
        Ticket::Standard(20.05),
        Ticket::VIP(50.80, "Joseph".to_owned()),
    ];

    for t in &ticketsVec {
        print_details(t);
    }
}

fn print_details(ticket: &Ticket) {
    match ticket {
        Ticket::Standard(num) => println!("[STANDARD] Ticket Price : {:?}", num),
        Ticket::Backstage(num, holder) => {
            println!(
                "[BACKSTAGE] Ticket Price : {:?} | Ticket Holder : {:?}",
                num, holder
            )
        }
        Ticket::VIP(num, holder) => {
            println!(
                "[VIP] Ticket Price : {:?} | Ticket Holder : {:?}",
                num, holder
            )
        }
    }
}
