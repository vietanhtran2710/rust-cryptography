use std::env;
pub mod caesar;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        println!("Usage: cargo run -- [methods] [-d/-e] [input] [key]")
    }
    else {
        let key = args[4].parse::<u16>().unwrap();
        let input = &args[3];
        if args[2] == "-d" {
            println!("{}", caesar::decrypt(input.to_string(), key));
        }
        else {
            println!("{}", caesar::encrypt(input.to_string(), key));
        }
    }
}
