fn is_triangle(a: i64, b: i64, c: i64) -> bool {
    if a+b>c && a+c>b && b+c>a {
        return true;
    }
    return false;
}

fn main() {
    println!("{}", is_triangle(1, 2, 2))
}
