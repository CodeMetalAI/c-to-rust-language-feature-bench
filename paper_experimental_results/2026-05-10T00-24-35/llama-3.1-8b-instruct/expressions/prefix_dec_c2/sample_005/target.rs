fn main() {
  let mut x = 5;

  let y = x - 1;
  if y!= 4 {
    println!("Test failed");
    std::process::exit(1);
  }
  if x!= 4 {
    println!("Test failed");
    std::process::exit(2);
  }

  x = 10;
  let y = x - 1;
  let z = 10;
  z -= 1;
  if y!= 9 {
    println!("Test failed");
    std::process::exit(3);
  }
  if x!= 9 {
    println!("Test failed");
    std::process::exit(4);
  }
  if z!= 9 {
    println!("Test failed");
    std::process::exit(5);
  }

  println!("Test passed");
}