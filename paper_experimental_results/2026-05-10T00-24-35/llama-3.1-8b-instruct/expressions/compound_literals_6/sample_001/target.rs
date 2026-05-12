fn main() {
  let a = "abc";
  let b = "abc";

  if a.chars().collect::<Vec<_>>()[0] != 'a' || a.chars().collect::<Vec<_>>()[1] != 'b' || a.chars().collect::<Vec<_>>()[2] != 'c' || a.chars().collect::<Vec<_>>()[3] != '\0' {
    return 1;
  }
  if b.chars().collect::<Vec<_>>()[0] != 'a' || b.chars().collect::<Vec<_>>()[1] != 'b' || b.chars().collect::<Vec<_>>()[2] != 'c' || b.chars().collect::<Vec<_>>()[3] != '\0' {
    return 2;
  }

  if a == b {
    return 3;
  }

  return 0;
}