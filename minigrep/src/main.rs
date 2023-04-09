use minigrep::Config;
use std::env;
use std::process;

fn main() {
    // This is a mini version of the grep command in linux
    // * Objectives
    // - Get Args
    // - Read the provided file
    // - Give the output

    // Declaring the args vector
    let args: Vec<String> = env::args().collect();

    // let query = &args[1];
    // let file_path = &args[2];

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem in parsing the args : {err}");
        process::exit(1);
    });

    println!("Query : {}", config.query);
    println!("File Path : {}", config.file_path);

    // Reading the contents of the file
    if let Err(e) = minigrep::run(config) {
        println!("Application error : {e}");
        process::exit(1);
    };
}
