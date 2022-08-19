extern crate num;

use num::integer::gcd;

pub fn check_key(a: u8, b: u8) -> bool {
    if gcd(a, 26) != 1 { return false; }
    else {
        if a == 1 && b == 0 { return false; }
    }
    return true;
}

fn get_inverse(a: u8) -> i16 {
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
    s_i1 as i16
}

pub fn decrypt(input: String, key_a: u8, key_b: u8) -> String {
    let a_1 = get_inverse(key_a);
    input.chars().map(|c| 
        if c.is_ascii_lowercase() {
            (97 + (a_1 * (c as i16 - 97 - key_b as i16)).rem_euclid(26) as u8) as char
        }
        else if c.is_ascii_uppercase() {
            (65 + (a_1 * (c as i16 - 65 - key_b as i16)).rem_euclid(26) as u8) as char
        }
        else {
            c
        }
    ).collect()
}

pub fn encrypt(input: String, key_a: u8, key_b: u8) -> String {
    input.chars().map(|c| 
        if c.is_ascii_lowercase() {
            (97 + ((c as u8 - 97) * key_a + key_b) % 26) as char
        } 
        else if c.is_ascii_uppercase() {
            (65 + ((c as u8 - 65) * key_a + key_b) % 26) as char
        }
        else {
            c
        }
    ).collect()
}