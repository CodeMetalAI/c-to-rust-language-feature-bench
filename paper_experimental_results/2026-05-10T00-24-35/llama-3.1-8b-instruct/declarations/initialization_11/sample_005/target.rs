fn main() {
  let w = [
    {a: [1, 0, 0], b: 0},
    {a: [2, 0, 0], b: 0},
  ];

  if w.len() != 2 {
    return;
  }

  if w[0].a[0] != 1 {
    return;
  }
  if w[0].a[1] != 0 {
    return;
  }
  if w[0].a[2] != 0 {
    return;
  }
  if w[0].b != 0 {
    return;
  }

  if w[1].a[0] != 2 {
    return;
  }
  if w[1].a[1] != 0 {
    return;
  }
  if w[1].a[2] != 0 {
    return;
  }
  if w[1].b != 0 {
    return;
  }

  println!("Success");
}