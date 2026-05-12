fn main() {
  let a: [i32; 3] = [10, 20, 30];

  if!a.is_reference()
    return 1;
  if!a.as_ref().is_array()
    return 2;

  if a[0]!= 10
    return 3;
  if a[1]!= 20
    return 4;
  if a[2]!= 30
    return 5;

  if std::mem::size_of_val(&a)!= 3 * std::mem::size_of::<i32>()
    return 6;
  if std::mem::align_of_val(&a)!= std::mem::align_of::<i32>()
    return 7;

  let s: [u8; 4] = *b"abc\0";
  if s[0] as char!= 'a'
    return 8;
  if s[1] as char!= 'b'
    return 9;
  if s[2] as char!= 'c'
    return 10;
  if s[3] as char!= '\0'
    return 11;

  return 0;
}

trait ArrayType {
  fn is_reference(&self) -> bool;
  fn is_array(&self) -> bool;
}

impl<T> ArrayType for [T] {
  fn is_reference(&self) -> bool {
    true
  }
  fn is_array(&self) -> bool {
    false
  }
}

impl<'a, T> ArrayType for &'a [T] {
  fn is_reference(&self) -> bool {
    false
  }
  fn is_array(&self) -> bool {
    true
  }
}