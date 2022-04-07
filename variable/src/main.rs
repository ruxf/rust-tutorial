#![allow(unused_doc_comments)]
fn main() {
    /**
     * 不可变变量和可变变量
     * 
     * 声明不可变变量
     * let a = 5;
     * 
     * 声明可变变量
     * let mut a = 5;
     */
    let mut a = 5;
    println!("{}", a);

    a = 6;
    println!("{}", a);

    /**
     * 变量解构
     */
    let (a, mut b): (bool, bool) = (true, false);
    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    assert_eq!(a, b);

    /**
     * 元组/切片/结构 解构
     */
    struct Struct {
        e: i32
    }

    let (a, b, c, d, e);
    (a, b) = (1, 2);
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct {e, ..} = Struct {e: 5};

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);

    /**
     * 变量遮蔽，即变量覆盖
     */

     let x = 5;
     let x = x + 1;
     {
         let x = x * 2;
         println!("作用域内的x: {}", x);
     }
     println!("x: {}", x);

     let letters = "abcdefg";
     println!("letters is: {}", letters);
     let letters = letters.len();
     println!("letters now is: {}", letters);

}
