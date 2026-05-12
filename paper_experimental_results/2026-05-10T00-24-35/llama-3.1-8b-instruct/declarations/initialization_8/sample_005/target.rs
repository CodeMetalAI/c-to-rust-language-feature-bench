fn main() {
  let s = "abc";
  let mut t = [0; 3];
  t.copy_from_slice("abc");

  if s.chars().zip(s.chars()).any(|(c1, c2)| c1!= c2) {
    return;
  }

  if t.iter().zip(t.iter()).any(|(c1, c2)| *c1!= *c2) {
    return;
  }

  s.get_mut(0).map(|c| *c = 'x');
  t[2] = 'y';

  if s.get(0).map(|c| *c)!= Some('x') {
    return;
  }

  if t[2]!= 'y' {
    return;
  }

  {
    let p = "abc";
    if p.chars().zip(p.chars()).any(|(c1, c2)| c1!= c2) {
      return;
    }
  }

  println!("all tests passed");
}