fn main() {
  let a = 1;
  let b = 2;
  let c = 2;

  if!(a < b && b < c) {
    std::process::exit(1);
  }

  if!((a < b) as i32 < c) {
    std::process::exit(2);
  }

  let a = 3;
  let b = 2;
  let c = 2;

  if!(a < b && b < c) {
    std::process::exit(3);
  }

  std::process::exit(0);
}