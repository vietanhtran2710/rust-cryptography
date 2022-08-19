pub fn decrypt(input: String, key: u8) -> String {
    input.chars().map(|c| 
        if c.is_ascii_lowercase() {
            let mut index = c as u8 - key;
            if index < 97 { index = 122 - (97 - index) }
            index as char
        }
        else if c.is_ascii_uppercase() {
            let mut index = c as u8 - key;
            if index < 65 { index = 90 - (65 - index) }
            index as char
        }
        else {
            c
        }
    ).collect()
}

pub fn encrypt(input: String, key: u8) -> String {
    input.chars().map(|c| 
        if c.is_ascii_lowercase() {
            (97 + (c as u8 - 97 + key) % 26) as char
        }
        else if c.is_ascii_uppercase() {
            (65 + (c as u8 - 65 + key) % 26) as char
        }
        else {
            c
        }
    ).collect()
}