fn main() {
  let a: [i32; 3] = [10, 20, 30];

  if!std::any::type_name::<&[i32]>().starts_with("&[i32")
  ||!std::any::type_name::<&[i32; 3]>().starts_with("&[i32; 3]")
  {
    return;
  }

  if a[0]!= 10 {
    return;
  }
  if a[1]!= 20 {
    return;
  }
  if a[2]!= 30 {
    return;
  }

  if std::mem::size_of_val(&a)!= 3 * std::mem::size_of::<i32>() {
    return;
  }
  if std::mem::align_of_val(&a)!= std::mem::align_of::<i32>() {
    return;
  }

  let s: [char; 4] = ['a', 'b', 'c', '\0'];
  let str: &str = "abc";

  if str.len()!= 3 {
    return;
  }
  if s[0]!= 'a' {
    return;
  }
  if s[1]!= 'b' {
    return;
  }
  if s[2]!= 'c' {
    return;
  }
  if s[3]!= '\0' {
    return;
  }
}