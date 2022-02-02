fn find_missing_letter(chars: &[char]) -> char {
    (chars[0]..=chars[chars.len()-1]).find(|c| !chars.contains(c)).unwrap()
}

fn main(){
    
}
