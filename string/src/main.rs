#![allow(unused_doc_comments)]
fn main() {
    /**
     * 字面量类型
     */
    let chinese: &str = "中国人";

    for i in chinese.chars() {
        println!("{}", i);
    }

    for i in chinese.bytes() {
        println!("{}", i);
    }

    /**
     * String 类型
     */

    let s: String = String::from("abcdefghijklmnopqrst");
    println!("{}", &s[2..6]);
    println!("{}", &s[..6]);
    println!("{}", &s[6..]);
}
