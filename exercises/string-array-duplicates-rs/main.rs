fn dup(arry: Vec<String>) -> Vec<String> {
    arry
        .iter()
        .map(|el| {
            let mut dup = el.chars().collect::<Vec<_>>();
            dup.dedup();
            dup
        })
        .collect::<Vec<Vec<char>>>()
        .iter()
        .map(|b| b.iter()
        .collect::<String>())
        .collect::<Vec<String>>()
}