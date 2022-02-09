fn spin_words(string: &str) -> String {
    let mut final_string = String::new();
    for word in string.split_whitespace() {
        if word.chars().count() >= 5 {
        // I know this is a little bit hacky
        // I'm new in rust :(
        // Adds a &String (which is coerced into &str) reversed into final_string variable
            final_string.push_str(&format!(" {}", word.chars().rev().collect::<String>())) 
        } 
        else {
        // If the word's length is not greater or equal than 5, just add the word to final_string variable
            final_string.push_str(&format!(" {}", word));
            continue;
        }
    }
    final_string[1..].to_string() // As it will return a white space as the first char, I will remove it

}


fn main(){
    spin_words("Hello worlds");
}