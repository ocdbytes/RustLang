#[derive(Debug, Clone, Copy)]
enum Position {
    Worker,
    Manager,
    Supervisor,
}

#[derive(Debug, Clone, Copy)]
struct Employee {
    position: Position,
    work_hours: i64,
}

fn print_it(em: Employee) {
    println!("{:?}", em);
}

pub(crate) fn main() {
    let me = Employee {
        position: Position::Worker,
        work_hours: 12,
    };

    println!("{:?}", me.position); // Worker
    println!("{:?}", me); // Employee { position: Worker, work_hours: 12 }

    // Clone & Copy :
    // clone and copy will allow us to clone the var and copy it whenever some function requires it to be used. It eliminates the general ownership transfer behavior and instead make a copy and send it to the function

    print_it(me);
    print_it(me);
    // NO ERROR
}
