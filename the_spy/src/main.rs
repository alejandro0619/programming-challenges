#![feature(slice_as_chunks)]
use std::{fs::{self, OpenOptions}, collections::HashMap, io::Write};
use serde_json;

fn main() {
    // read the file.
    let content = fs::read_to_string("spy.in").expect("The file were not found");
    let mut row_number = 0;
    let mut column_number = 0;
    let mut matrix_inline: Vec<&str> = Vec::new();
    let mut out_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("spy.out").unwrap();

    let code_matrix = r#"{ 
        "0": " ", "1": "A", "2": "B", "3": "C", "4": "D", "5": "E",
        "6": "F", "7": "G", "8": "H", "9": "I", "10": "J", "11": "K",
        "12": "L", "13": "M", "14": "N", "15": "Ñ", "16": "O",  "17": "P",
        "18": "Q", "19": "R", "20": "S", "21": "T", "22": "U", "23": "V",
        "24": "W", "25": "X", "26": "Y", "27": "Z", "90": "0", "91": "9",
        "92": "8", "93": "7", "94": "6", "95": "5", "96": "4", "97": "3",
        "98": "2", "99": "1", "61": ",", "62": "."
    }"#;
    let code_matrix: HashMap<u8, &str> = serde_json::from_str(code_matrix).unwrap();
    // Gets the row and column number
    content.lines().for_each(|line| {
        row_number += 1;
        let col_number = line.split_whitespace().collect::<Vec<&str>>().len();
        if col_number > column_number {
            column_number = col_number;
        }
    });
    // Check if it meets the requirements.
    if (column_number >= 10 || !column_number % 2 == 0) || (row_number <= 1 || row_number >= 10) {
        println!("No, there's no message.")
    } else {
        // Push every character found to the vector, except those that are whitespaces.
        content.lines().for_each(|line| {
            line.split_whitespace().for_each(|character| {
                if character != " " {
                    matrix_inline.push(character)
                }
            });
        });
        // Split into chunks of two
        let (split_matrix, _) = matrix_inline.as_chunks::<2>();
        let mut deciphered_msg = String::new();
        for pair in split_matrix {
            let str_pair = format!("{}{}", pair[0], pair[1]);
            let parsed_pair = str_pair.parse::<u8>().unwrap();
            if !code_matrix.contains_key(&parsed_pair) {
                continue
            } else {
                let letter = code_matrix.get(&parsed_pair).unwrap();
                deciphered_msg.push_str(letter);
            }
        }
        out_file.write_all(deciphered_msg.as_bytes()).unwrap();
    }
    
}
