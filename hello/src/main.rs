fn main() {
    let germany: &str = "Grüß Gott!";
    let chinese: &str = "你好，世界！";
    let english: &str = "hello, world!";
    let regions: [&str; 3] = [germany, chinese, english];
    for region in regions.iter() {
        println!("{}", &region);
    }
}
