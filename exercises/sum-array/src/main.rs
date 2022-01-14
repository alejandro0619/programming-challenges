fn sum_array(a: [i32; 5]){
  let mut account = 0;
  
  for i in a.iter() {
    account += i;
  }
  println!("{}", account)
}
fn main() {
    sum_array([1, 2, 3, 4, 5]);
}
