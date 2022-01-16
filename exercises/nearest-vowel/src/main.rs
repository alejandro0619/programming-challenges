fn distance_nearest_vowel(s: &str) {
  let s = s.to_lowercase();
  let bytes = s.as_bytes();
  let vowels: [u8; 5] = [b'a', b'e', b'i', b'o', b'u'];
  for (i, c) in bytes.iter().enumerate(){
    if vowels.contains(&c){
      println!("The distance from the beginning to the first vowel is: {}", i);
      break;
    }
  }
}

fn main() {
  distance_nearest_vowel("Hhhfgfyhfgyfygf fyu")
}
