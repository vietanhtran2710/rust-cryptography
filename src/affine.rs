extern crate num;

use num::integer::gcd;

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z',
];

static ASCII_UPPER: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 
    'F', 'G', 'H', 'I', 'J', 
    'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S', 'T', 
    'U', 'V', 'W', 'X', 'Y', 
    'Z',
];

pub fn check_key(a: u16, b: u16) -> bool {
    if gcd(a, 26) != 1 { return false; }
    else {
        if a == 1 && b == 0 { return false; }
    }
    return true;
} 

pub fn decrypt(input: String, key_a: u16, key_b: u16) -> String {
    "".to_string()
}

pub fn encrypt(input: String, key_a: u16, key_b: u16) -> String {
    input.chars().map(|c| 
        if c.is_ascii_lowercase() {
            let index = ASCII_LOWER.iter().position(|&r| r == c).unwrap();
            let new_index = (index * key_a as usize + key_b as usize) % 26;
            ASCII_LOWER[new_index]
        }
        else if c.is_ascii_uppercase() {
            let index = ASCII_UPPER.iter().position(|&r| r == c).unwrap();
            let new_index = (index * key_a as usize + key_b as usize) % 26;
            ASCII_UPPER[new_index]
        }
        else {
            c
        }
    ).collect()
}