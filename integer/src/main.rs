fn main() {
    // float
    let x = 2.0;
    let y: f32 = 3.0;
    
    // operation
    let sum = 5 + 10;
    let difference = 33.5 - 3.2;
    let product = 4 * 34;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    let twenty = 20;
    let twenty_one: i32 = 21;
    let twenty_two = 22i32;

    let addition = twenty + twenty_one + twenty_two;
    println!("additon: {} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    let forty_twos = [
        42.0,
        42f32,
        42.0_f32,
    ];
    println!("{:.2}", forty_twos[0]);

    // 断言 0.1 + 0.2 = 0.3
    // assert!(0.1 + 0.2 == 0.3);

    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("0.1 + 0.2: {:x}", (xyz.0 + xyz.0).to_bits());
    println!("0.3: {:x}", (xyz.2).to_bits());
    println!();

    // assert!(abc.0 + abc.1 == abc.2);
    // assert!(xyz.0 + xyz.1 == xyz.2);

    // NaN
    let x = (-42.0_f32).sqrt();
    // assert_eq!(x, x);
    if x.is_nan() {
        println!("The value is NaN");
    }
    
    // range
    for i in 1..5 {
        println!("i: {}", i);
    }

    for i in 1..=5 {
        println!("i: {}", i);
    }

    for c in 'a'..'z' {
        println!("c: {}", c);
    }
}
