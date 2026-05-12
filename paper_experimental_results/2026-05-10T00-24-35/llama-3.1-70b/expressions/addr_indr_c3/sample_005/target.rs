fn main() {
  let x: i32 = 7;
  let p = &x;

  if p as *const i32!= &x as *const i32 {
    std::process::exit(1);
  }

  if &*p as *const i32!= p as *const i32 {
    std::process::exit(2);
  }

  let a = [10, 20, 30];

  if &a[0] as *const i32!= &a[0] as *const i32 {
    std::process::exit(3);
  }
  if &a[1] as *const i32!= (&a[0] as *const i32).offset(1) {
    std::process::exit(4);
  }
  if &a[2] as *const i32!= (&a[0] as *const i32).offset(2) {
    std::process::exit(5);
  }

  if unsafe { *(&a[0] as *const i32).offset(1) }!= 20 {
    std::process::exit(6);
  }
}