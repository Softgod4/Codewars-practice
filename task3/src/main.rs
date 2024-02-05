fn longest(a1: &str, a2: &str) -> String {
    let concat = format!("{}{}", a1, a2);
    let _charvec: Vec<char> = concat.chars().collect();
    let mut result: Vec<char> = vec![];
    for n in 0.._charvec.len() {
        if !result.contains(&_charvec[n]) {
            result.push(_charvec[n])
        }
    }
    result.sort();
    return result
        .into_iter()
        .map(|x| x.to_string())
        .collect::<String>();
}

fn main() {
    println!("{}", longest("aretheyhere", "yestheyarehere"));
}
