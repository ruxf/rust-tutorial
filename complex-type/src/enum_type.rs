enum PokerSuit {
  Clubs,
  Spades,
  Diamonds,
  Hearts,
}

enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

// OPtion 空值枚举
enum Option<T> {
  Some(T),
  None,
}

fn define_enum() {
  let some_number = Some(5);
  let some_string = Some("a string");

  // 如果使用 None 值，则需要使用 Option<T> 来定义 Some 的类型
  let absent_number: Option<i32> = None;
}

fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}

fn main() {
  let heart = PokerSuit::Hearts;
  let diamond = PokerSuit::Diamonds;

  print_suit(heart);
  print_suit(diamond);

  // 打印 Message 枚举
  let m1 = Message::Quit;
  let m2 = Message::Move{x: 1, y: 1};
  let m3 = Message::ChangeColor(255, 255, 127);

  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);
}

fn print_suit(card: PokerSuit) {
  println!("{:?}", card);
}

fn new (stream: TcpStream) {
  let mut s = stream;
  if tls {
    s = negotiate_tls(stream);
  }
}