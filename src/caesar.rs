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

pub fn decrypt(input: String, key: u16) -> String {
    input.chars().map(|c| 
        if c.is_ascii_lowercase() {
            let mut index: i16 = ASCII_LOWER.iter()
            .position(|&r| r == c)
            .unwrap() as i16 - key as i16;
            if index < 0 { index = 26 + index }
            ASCII_LOWER[(index as usize) % 26]
        }
        else if c.is_ascii_uppercase() {
            let mut index = ASCII_UPPER.iter()
            .position(|&r| r == c)
            .unwrap() as i16 - key as i16;
            if index < 0 { index = 26 + index }
            ASCII_UPPER[(index as usize) % 26]
        }
        else {
            c
        }
    ).collect()
}

pub fn encrypt(input: String, key: u16) -> String {
    input.chars().map(|c| 
        if c.is_ascii_lowercase() {
            let index = ASCII_LOWER.iter().position(|&r| r == c).unwrap();
            ASCII_LOWER[(index + key as usize) % 26]
        }
        else if c.is_ascii_uppercase() {
            let index = ASCII_UPPER.iter().position(|&r| r == c).unwrap();
            ASCII_UPPER[(index + key as usize) % 26]
        }
        else {
            c
        }
    ).collect()
}