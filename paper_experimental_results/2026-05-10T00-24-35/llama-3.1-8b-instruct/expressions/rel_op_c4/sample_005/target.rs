fn main() {
  let x = 10;
  let y = 20;

  let px = &x;
  let py = &y;

  let px_end = px;
  let py_end = py;

  if px != &x {
    return 1;
  }

  if px_end > px {
    return 2;
  }

  if px_end == (&x + 1) {
    return 3;
  }

  if py_end > py {
    return 4;
  }

  return 0;
}