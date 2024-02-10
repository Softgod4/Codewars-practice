fn wave(s: &str) -> i32 {
    let mut wave_vector: Vec<String> = Vec::new();
    let mut secondary: Vec<String> = Vec::new();
    let split_str = s.split("");
    for (i, item) in split_str.enumerate() {
        for j in 0..item.len() {
            if j == i {
            secondary.push(j.to_string());
            }
        }
    }
    for i in secondary {
        println!("{}", i);
    }
    return 1;
}

fn main() {
    println!("{}", wave("hello"));
}

