fn main() {
    let a = [1, 2, 3, 4];
    for item in a {
        println!("{}", item);
    }

    println!("{:?}", a);

    // 如果是复合类型，即动态数组，则需要使用不可变引用
    let A = vec![1, 2, 3, 4];
    for item in &A {
        println!("{}", item);
    }

    println!("{:?}", A);

    // 如果需要使用索引，那么需要使用 iter().enumerate() 转换成元组
    println!("{:?}", a.iter());

    for (index, item) in a.iter().enumerate() {
        println!("{}: {}", index, item);
    }
}
