
















pub fn get_first_word(line: &str) -> &str {

let mut index = 0;
    for i in line.chars() {

        if i == ' ' {
            return &line[0..index];
        }
        index = index + 1;
    }

return line;
}




















