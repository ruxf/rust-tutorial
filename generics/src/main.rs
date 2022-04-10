fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    println!("{}", add(3_i8, 4_i8));

    // 打印最大值
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}
