fn main() {
  let mut a = 1;
  let mut b = 2;
  let mut c = 3;
  let mut d = 4;
  if (a < b) as i32 == (c < d) as i32 != 1 {
    return;
  }

  a = 2;
  b = 1;
  c = 4;
  d = 3;
  if (a < b) as i32 == (c < d) as i32 != 1 {
    return;
  }

  a = 1;
  b = 2;
  c = 4;
  d = 3;
  if (a < b) as i32 == (c < d) as i32 != 0 {
    return;
  }

  a = 2;
  b = 1;
  c = 3;
  d = 4;
  if (a < b) as i32 == (c < d) as i32 != 0 {
    return;
  }

  a = 1;
  b = 2;
  c = 3;
  d = 4;
  if (a < b) as i32 == (c < d) as i32 != 1 {
    return;
  }
}