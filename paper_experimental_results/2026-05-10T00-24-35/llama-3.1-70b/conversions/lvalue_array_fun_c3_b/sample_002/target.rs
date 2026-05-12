fn main() {
  let a = [10, 20, 30];

  if!a.is_array() {
    return 1;
  }
  if!(&a).is_array() {
    return 2;
  }

  if a[0]!= 10 {
    return 3;
  }
  if a[1]!= 20 {
    return 4;
  }
  if a[2]!= 30 {
    return 5;
  }

  if a.len()!= 3 || a.len() * std::mem::size_of::<i32>()!= std::mem::size_of_val(&a) {
    return 6;
  }
  if std::mem::align_of::<[i32; 3]>()!= std::mem::align_of::<i32>() {
    return 7;
  }

  let s = "abc".to_owned().into_bytes();
  if s[0]!= b'a' {
    return 8;
  }
  if s[1]!= b'b' {
    return 9;
  }
  if s[2]!= b'c' {
    return 10;
  }
  if s.len() < 3 {
    return 11;
  }
  if s.len() > 3 && s[3]!= b'\0' {
    return 12;
  }

  return 0;
}