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

fn get_inverse(a: u16) -> i32 {
    let (mut r_i1, mut r_i) = (a as i32, 26 as i32);
    let mut remainder = 1;
    let (mut s_i1, mut s_i) = (1, 0);
    while remainder != 0 {
        remainder = r_i1 % r_i;
        let q = r_i1 / r_i;
        let new_s = s_i1 - q * s_i;
        r_i1 = r_i; r_i = remainder;
        s_i1 = s_i; s_i = new_s;    
    }
    s_i1
}

pub fn decrypt(input: String, key_a: u16, key_b: u16) -> String {
    let a_1 = get_inverse(key_a);
    input.chars().map(|c| 
        if c.is_ascii_lowercase() {
            let index = ASCII_LOWER.iter().position(|&r| r == c).unwrap();
            let new_index: i32 = (a_1 * (index as i32 - key_b as i32)).rem_euclid(26);
            ASCII_LOWER[new_index as usize]
        }
        else if c.is_ascii_uppercase() {
            let index = ASCII_UPPER.iter().position(|&r| r == c).unwrap();
            let new_index = (a_1 * (index as i32 - key_b as i32)).rem_euclid(26);
            ASCII_UPPER[new_index as usize]
        }
        else {
            c
        }
    ).collect()
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