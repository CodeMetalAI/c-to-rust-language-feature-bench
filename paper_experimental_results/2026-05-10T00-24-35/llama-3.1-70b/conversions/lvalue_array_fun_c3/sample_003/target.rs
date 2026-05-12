fn main() {
  let a = [10, 20, 30];

  if!a.is_slice_of_type::<i32>() {
    return 1;
  }
  if!(&a).is_array_of_type::<i32, 3>() {
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

  if std::mem::size_of_val(&a)!= 3 * std::mem::size_of::<i32>() {
    return 6;
  }
  if std::mem::align_of::<[i32; 3]>()!= std::mem::align_of::<i32>() {
    return 7;
  }

  let s: [u8; 4] = *b"abc";
  if!std::str::from_utf8(&s).unwrap().is_ascii() {
    return 8;
  }
  if s[0]!= b'a' {
    return 9;
  }
  if s[1]!= b'b' {
    return 10;
  }
  if s[2]!= b'c' {
    return 11;
  }
  if s[3]!= 0 {
    return 12;
  }

  return 0;
}

trait TypeId<T> {
  fn is_type(&self) -> bool;
}

impl<T> TypeId<T> for [T] {
  fn is_type(&self) -> bool {
    true
  }
}

impl<T, const N: usize> TypeId<T> for [T; N] {
  fn is_type(&self) -> bool {
    true
  }
}

trait SliceTypeId {
  fn is_slice_of_type<T>(&self) -> bool;
}

impl<T> SliceTypeId for [T] {
  fn is_slice_of_type<U>(&self) -> bool {
    std::any::type_name::<T>() == std::any::type_name::<U>()
  }
}

trait ArrayTypeId {
  fn is_array_of_type<T, const N: usize>(&self) -> bool;
}

impl<T> ArrayTypeId for [T] {
  fn is_array_of_type<U, const M: usize>(&self) -> bool {
    std::any::type_name::<T>() == std::any::type_name::<U>() && self.len() == M
  }
}