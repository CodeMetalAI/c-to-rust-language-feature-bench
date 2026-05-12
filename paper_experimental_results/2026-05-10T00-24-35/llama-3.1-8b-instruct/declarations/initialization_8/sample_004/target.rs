fn main() {
  let s = "abc";
  let mut t = String::from("abc");

  match (s.chars().nth(0), s.chars().nth(1), s.chars().nth(2), s.chars().last()) {
      ('a', 'b', 'c', Some('\0')) => (),
      _ => println!("Error: s is not a C-style string."},
  };

  match (t.chars().nth(0), t.chars().nth(1), t.chars().nth(2)) {
      ('a', 'b', 'c') => (),
      _ => println!("Error: t is not a C-style string."),
  };

  s.replace_range(0..1, "x");
  t.replace_range(2..3, "y");

  match (s.chars().nth(0), s.chars().last()) {
      ('x', Some('\0')) => (),
      _ => println!("Error: s is not a C-style string."),
  };

  match (t.chars().nth(2), t.chars().last()) {
      ('y', Some('\u{0}')) => (),
      _ => println!("Error: t is not a C-style string."),
  };

  let p = "abc";
  match (p.chars().nth(0), p.chars().nth(1), p.chars().nth(2), p.chars().last()) {
      ('a', 'b', 'c', Some('\0')) => (),
      _ => println!("Error: p is not a C-style string."),
  };

  std::process::exit(0);
}