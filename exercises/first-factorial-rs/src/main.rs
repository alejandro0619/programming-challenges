fn first_factorial(mut num: u32) -> u32 {
    let mut acc: u32 = 1;
    while num >= 1 {
        acc = acc * num;
        num = num - 1;
    }
    acc
}
fn main() {
    println!("{}", first_factorial(5));
}
