extern crate num;

pub fn decrypt(_input: String, _key: String) -> String {
    "".to_string()
}

pub fn encrypt(input: String, key: String) -> String {
    input.chars().enumerate().map(|(index, _c)|
        {
            (&key[(index * input.len())..((index + 1) * input.len())]
            .chars().enumerate().map(|(i, k)|
                {
                    (k as u16 - 97) * (input.chars().nth(i).unwrap() as u16 - 97)
                }
            ).into_iter().sum::<u16>() % 26 + 97) as u8 as char
        }
    ).collect()
}