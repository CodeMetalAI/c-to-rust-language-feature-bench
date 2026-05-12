fn main() {
  let mut x = 7;
  let p: &i32 = &x;

  if std::ptr::eq(&x, p) {
    return 1;
  }

  let y = *p;
  if std::ptr::eq(&y, p) {
    return 2;
  }

  let mut a = [10, 20, 30];

  if std::ptr::eq(&a[0], &a[0]) {
    return 3;
  }
  if std::ptr::eq(&a[1], &a[1]) {
    return 4;
  }
  if std::ptr::eq(&a[2], &a[2]) {
    return 5;
  }

  if a[1] != 20 {
    return 6;
  }

  0
}