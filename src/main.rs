use std::env;
pub mod caesar;
pub mod affine;
pub mod vigenere;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: cargo run -- [methods] [-d/-e] [input] [keys]");
        return;
    }
    if args[1] == "caesar" {
        if args.len() < 5 {
            println!("Usage: cargo run -- caesar [-d/-e] [input] [key]");
            return;
        }
        let key: u8 = args[4].parse::<u8>().unwrap();
        let input = &args[3];
        if args[2] == "-d" {
            println!("{}", caesar::decrypt(input.to_string(), key));
        }
        else {
            println!("{}", caesar::encrypt(input.to_string(), key));
        }
    }
    else if args[1] == "affine" {
        if args.len() < 6 {
            println!("Usage: cargo run -- affine [-d/-e] [input] [key_a] [key_b]");
            return;
        }
        let key_a = args[4].parse::<u8>().unwrap();
        let key_b = args[5].parse::<u8>().unwrap();
        if affine::check_key(key_a, key_b) {
            let input = &args[3];
            if args[2] == "-d" {
                println!("{}", affine::decrypt(input.to_string(), key_a, key_b));
            }
            else {
                println!("{}", affine::encrypt(input.to_string(), key_a, key_b));
            }
        }
        else {
            println!("Invalid keys! Key_a and 26 must be coprime. If key_a = 1 => key_b != 0");
        }
    }
    else if args[1] == "vigenere" {
        if args.len() < 4 {
            println!("Usage: cargo run -- vigenere [-d/-e] [input] [key]");
            return;
        }
        let key = args[4].to_lowercase();
        let input = args[3].to_lowercase();
        if args[2] == "-d" {
            println!("{}", vigenere::decrypt(input, key));
        }
        else {
            println!("{}", vigenere::encrypt(input, key));
        }
    }
}
