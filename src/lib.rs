use std::env::{self, temp_dir};






pub fn get_first_word(line: &str) -> &str {
let mut index = 0;
    if line.get(0..1).unwrap() == "!" {
        for i in line.chars() {
            if i == ' ' {
                return &line[0..index];
            } else { index += 1; }
        }
    }
&line[0..line.len()]
}




















