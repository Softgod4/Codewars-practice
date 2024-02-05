fn summation(n: i32) -> i32 {
    let mut i: i32 = 0;
    let mut result: i32 = 0;
    while i < n {
        i += 1;
        result += i;
    }
    return result;
}

fn main() {
    println!("{}", summation(1));
}
