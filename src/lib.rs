
















pub fn get_first_word(line: &str) -> &str {
let mut index = 0;

    if line.get(0..1).unwrap() == "!" {
        for i in line.chars() {
            if i == ' ' {
                return &line[0..index];
            } else { index = index + 1; }
        }
    }

return &line[0..line.len()];
}




















