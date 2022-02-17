fn main() {
    println!(
        "{}",
        alphabet_position("The sunset sets at twelve o' clock.")
    );
}

fn alphabet_position(text: &str) -> String {
    let text = text
        .to_owned()
        .to_lowercase()
        .chars()
        .map(|c| match c {
            'a'..='z' => c,
            _ => ' ',
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<String>();

    return text
        .chars()
        .map(|x| format!("{}", (x as u32) - 96))
        .collect::<Vec<String>>()
        .join(" ");
}
