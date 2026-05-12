fn sum(p: &Vec<i32>, n: i32) -> i32 {
  let mut s = 0;
  for (i, x) in p.iter().enumerate() {
    if i as i32 < n {
      s += *x;
    }
  }
  return s;
}

fn main() {
  let a = vec![1, 2];
  let b = vec![3, 4, 5];

  if a.len() != 2 {
    println!("Error: a has incorrect length");
    std::process::exit(1);
  }
  if b.len() != 3 {
    println!("Error: b has incorrect length");
    std::process::exit(2);
  }

  if sum(&a, 2) != 3 {
    println!("Error: sum of a is incorrect");
    std::process::exit(3);
  }
  if sum(&b, 3) != 12 {
    println!("Error: sum of b is incorrect");
    std::process::exit(4);
  }

  {
    let pa = &a;
    let pb = &b;
    if pa[1] != 2 {
      println!("Error: a[1] is incorrect");
      std::process::exit(5);
    }
    if pb[2] != 5 {
      println!("Error: b[2] is incorrect");
      std::process::exit(6);
    }
  }

  println!("All checks passed");
  std::process::exit(0);
}