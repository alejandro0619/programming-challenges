fn reverse_string(string: &str) -> String {
  let mut reversed_string = String::new();
  
  // chars().rev() returns an iterator -> loop through it.
  // we convert rc (which is char) to a String using to_string() method.
  // Thus .push_str() method receives only &str as parameters
  // We pass a &String a String refence (rc won't be in scope anymore btw)
  // And Rust coerced &String to &str, .push_str() method is satisfied
  for rc in string.chars().rev() { 
    reversed_string.push_str(&rc.to_string()); 
  }
  reversed_string
}
fn main() {
  println!("{}",reverse_string("hello"));
}
