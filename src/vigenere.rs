static TABLE: [&str; 26] = [
    "abcdefghijklmnopqrstuvwxyz",
    "bcdefghijklmnopqrstuvwxyza",
    "cdefghijklmnopqrstuvwxyzab",
    "defghijklmnopqrstuvwxyzabc",
    "efghijklmnopqrstuvwxyzabcd",
    "fghijklmnopqrstuvwxyzabcde",
    "ghijklmnopqrstuvwxyzabcdef",
    "hijklmnopqrstuvwxyzabcdefg",
    "ijklmnopqrstuvwxyzabcdefgh",
    "jklmnopqrstuvwxyzabcdefghi",
    "klmnopqrstuvwxyzabcdefghij",
    "lmnopqrstuvwxyzabcdefghijk",
    "mnopqrstuvwxyzabcdefghijkl",
    "nopqrstuvwxyzabcdefghijklm",
    "opqrstuvwxyzabcdefghijklmn",
    "pqrstuvwxyzabcdefghijklmno",
    "qrstuvwxyzabcdefghijklmnop",
    "rstuvwxyzabcdefghijklmnopq",
    "stuvwxyzabcdefghijklmnopqr",
    "tuvwxyzabcdefghijklmnopqrs",
    "uvwxyzabcdefghijklmnopqrst",
    "vwxyzabcdefghijklmnopqrstu",
    "wxyzabcdefghijklmnopqrstuv",
    "xyzabcdefghijklmnopqrstuvw",
    "yzabcdefghijklmnopqrstuvwx",
    "zabcdefghijklmnopqrstuvwxy",
];

pub fn decrypt(input: String, key: String) -> String {
    input.chars().enumerate().map(|(index, c)| 
        {
            let row = key.chars().nth(index % key.len()).unwrap() as usize - 97;
            (TABLE[row].chars().position(|r| r == c).unwrap() as u8 + 97) as char
        }
    ).collect()
}

pub fn encrypt(input: String, key: String) -> String {
    input.chars().enumerate().map(|(index, c)| 
        {
            let row = key.chars().nth(index % key.len()).unwrap() as usize - 97;
            let col = c as usize - 97;
            TABLE[row].chars().nth(col).unwrap()
        }
    ).collect()
}