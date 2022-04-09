enum Direction {
  East,
  South,
  West,
  North,
}

fn main() {
  let direction = Direction::South;

  match direction {
    Direction::East => println!("East matched"),
    Direction::South => println("South matched"),
    Direction::West | direction::North => println!("West or North matched"),
    _ => println!("Default matched"),
  };
}