
use std::io;

pub fn calculate_total_payment(wh: u32, ch: u32)-> u32{
  wh * ch
}

pub fn display(payment: u32) {
  println!("The total payment for your work is: {}",payment);
}

pub fn get_user_input() -> (u32, u32) {
  let (worked_hours, cost_per_hour) = 'main: loop {
    println!("Input the hours you worked and the cost per hour separated by a space");

  let mut input = String::new();
  io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");
  
  if input.contains('-') {
    println!("The salary must be greater than 0!");
    continue 'main;
  } else {
    let input_split: Vec<&str> = input.split(' ').collect();
  let mut i = 0;
  let (worked_hours, cost_per_hour) = 'splitting_loop: loop {
    if i <= input_split.len() && input_split.len() > 1  {
      i = i + 1; 
      break 'splitting_loop (input_split[ i - 1 ], input_split[ i ])
    } else {
      println!("Please, enter two values!");
      continue 'main;
    }
  };
  // worked hours parse
  let wh: u32 = match worked_hours.trim().parse() {
    Ok(number_parsed) => {
      number_parsed
    },
    Err(_) => {
      println!("The number couldn't be parsed, please enter again");
      continue 'main
    }
  };
  // cost per hour parse
  let ch: u32 = match cost_per_hour.trim().parse(){
    Ok(number_parsed) => {
      number_parsed
    }
    Err(_) => {
      println!("The number couldn't be parsed, please enter again");
      continue 'main
    }
  };

  break 'main (wh, ch)
    }
  };
  (worked_hours, cost_per_hour)
}