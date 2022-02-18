fn main() {
    println!("{}", solution(10)); // prints 23
}
fn solution(num : i32) -> i32 {
    return (0..num)
    .filter(|n| n % 3 == 0 || n % 5 == 0)
    .fold(0, |acc, n| n + acc);
}