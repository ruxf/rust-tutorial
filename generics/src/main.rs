fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
fn main() {
    println!("{}", add(3_i8, 4_i8));
}
