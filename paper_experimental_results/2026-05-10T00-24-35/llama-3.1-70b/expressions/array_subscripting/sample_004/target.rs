fn main() {
  let mut x: [[i32; 5]; 3] = [[0; 5]; 3];

  for i in 0..3 {
    for j in 0..5 {
      x[i][j] = 1 * i as i32 + j as i32;
    }
  }

  for i in 0..3 {
    for j in 0..5 {
      let a = x[i][j];
      let b = unsafe { *(((x.as_ptr() as *const i32).add(5 * i)).add(j)) };
      if a!= b {
        std::process::exit(1);
      }
    }
  }

  let p0 = &x[0][0] as *const i32;
  let p1 = &x[1][0] as *const i32;
  let diff = p1 as isize - p0 as isize;
  if diff!= 5 {
    std::process::exit(2);
  }

  println!("PASS");
}