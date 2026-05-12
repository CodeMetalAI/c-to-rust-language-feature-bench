fn main() {
  let x = 10;
  let y = 20;

  let px = &x as *const i32;
  let py = &y as *const i32;

  let px_end = px.offset(1);
  let py_end = py.offset(1);

  if px as *const i32!= &x as *const i32 {
    std::process::exit(1);
  }

  if!(px_end > px) {
    std::process::exit(2);
  }

  if px_end!= (&x as *const i32).offset(1) {
    std::process::exit(3);
  }

  if!(py_end > py) {
    std::process::exit(4);
  }

  std::process::exit(0);
}