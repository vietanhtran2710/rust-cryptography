// static ASCII_LOWER: [char; 26] = [
//     'a', 'b', 'c', 'd', 'e', 
//     'f', 'g', 'h', 'i', 'j', 
//     'k', 'l', 'm', 'n', 'o',
//     'p', 'q', 'r', 's', 't', 
//     'u', 'v', 'w', 'x', 'y', 
//     'z',
// ];

// static ASCII_UPPER: [char; 26] = [
//     'A', 'B', 'C', 'D', 'E', 
//     'F', 'G', 'H', 'I', 'J', 
//     'K', 'L', 'M', 'N', 'O',
//     'P', 'Q', 'R', 'S', 'T', 
//     'U', 'V', 'W', 'X', 'Y', 
//     'Z',
// ];

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
    "".to_string()
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