fn parse(string: &str)-> Vec<u32> { 
    let mut val = 0u32;
    let mut result: Vec<u32> = Vec::new();
      for c in string.as_bytes() {
        match c {
          b'i' => val += 1,
          b'd' => val -= 1,
          b's' => val *= val,
          b'o' => result.push(val),
          _ => (),
        } 
      }
    result
  }
  
  fn main() {
      parse("iiisdoso");
      
}