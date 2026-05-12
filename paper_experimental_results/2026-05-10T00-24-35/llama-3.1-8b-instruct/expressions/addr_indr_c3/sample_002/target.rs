fn main() {
  let x = 7;
  let p = &x;

  if std::mem::transmute::<&int, &int>(&x) != p {
    return 1;
  }

  let a = [10, 20, 30];

  if std::mem::transmute::<&[int; 3], &[int; 3]>(a.as_ptr()) == a.as_ptr() {
    if std::mem::transmute::<&[int; 3], &[int; 3]>(a.as_ptr().offset(1)) == a.as_ptr().offset(1) {
      if std::mem::transmute::<&[int; 3], &[int; 3]>(a.as_ptr().offset(2)) == a.as_ptr().offset(2) {
        if a[1] == 20 {
          return 0;
        }
      }
    }
  }

  panic!("Tests failed");
}