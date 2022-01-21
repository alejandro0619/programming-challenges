use std::io;

fn get_input() -> u32 {
  let input_parsed = loop {
    let mut input = String::new();
    println!("This program will loop from 1 to the limit you enter, showing only those odds numbers");
    println!("Enter a limit...");
    io::stdin()
      .read_line(&mut input)
      .expect("Couldn't read the input.");
  
    // check if it's negative nor is an empty string
    if !input.contains('-') && input.is_empty(){ 
      // Eliminate possibles whitespaces and attempt to parse 
      match input.trim().parse() { 
        // If it's Ok break the loop with a return value
        Ok(n) => {
          break n
        },
        Err(_) => {
          // If it's any kind of error, jump to the next loop
          println!("Please enter a number");
          continue;
        }
      };
    } else {
      println!("The limit must be greater than zero");
      continue;
    }
  };

  input_parsed
}

fn loop_though_user_input(number_limit: u32) {
  // loop from 0 to the limit
  for n in 0..number_limit + 1 {
    // Check if it's odd
    if n % 2 != 0 {
      println!("{}", n)
    } else {
      continue;
    }
  }
}
fn main() {
    let number = get_input();
    loop_though_user_input(number);
}
