#![allow(unused)]
#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

fn main() {
    /**
     * 通过添加 #[derive(Debug)] 特征打印结构体
     */
    let rectangle = Rectangle {
        width: 30,
        height: 60,
    };
    // 正常输出：Rectangle { width: 30, height: 60 }
    // println!("{:?}", rectangle);
    // 格式化输出
    println!("{:#?}", rectangle);
}
