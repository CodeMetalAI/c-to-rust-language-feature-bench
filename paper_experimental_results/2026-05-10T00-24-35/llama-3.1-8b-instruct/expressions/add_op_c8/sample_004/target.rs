fn main() {
  let mut a = [10, 20, 30, 40, 50];

  let p0 = &a[0];
  let p2 = p0.offset(2);
  let p4 = p2.offset(2);
  let p_last = &a[4];
  let p_end = p_last.offset(1);

  if *p2 != 30 {
    println!("Error 1: *p2 != 30");
    return;
  }
  if *p4 != 50 {
    println!("Error 2: *p4 != 50");
    return;
  }

  let p_back = p4.offset(-3);
  if *p_back != 20 {
    println!("Error 3: *p_back != 20");
    return;
  }

  let p_from_end = p_end.offset(-1);
  if p_from_end != p_last {
    println!("Error 4: p_from_end != p_last");
    return;
  }
  if *p_from_end != 50 {
    println!("Error 5: *p_from_end != 50");
    return;
  }

  if p0.add(5) != p_end {
    println!("Error 6: p0.add(5) != p_end");
    return;
  }

  println!("All checks passed");
}