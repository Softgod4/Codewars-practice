fn fake_bin(s: &str) -> String {
    let mut notfive: Vec<String> = Vec::new();
    for i in s.chars() {
        let z = (i.to_string()).parse::<i32>().unwrap();
        if z < 5 {
            notfive.push("0".to_string());
        } else {
            notfive.push("1".to_string())
        } 
    }
    let result = notfive.iter().collect::<String>();
    return result; 
}

fn main() {
    println!("{}", fake_bin("45385593107843568"));
}
