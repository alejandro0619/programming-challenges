fn solution(word: &str, ending: &str) -> bool {
    word.to_lowercase().ends_with(&ending.to_lowercase())
}
fn main() {
    solution(&String::from("abc"), &String::from("bc"));
}
