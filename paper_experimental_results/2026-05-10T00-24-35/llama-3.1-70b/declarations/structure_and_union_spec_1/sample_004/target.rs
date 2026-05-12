fn main() {
  struct V {
    i_j: (i32, i32),
    k_l: (i64, i64),
    m: i32,
  }

  let mut v1 = V {
    i_j: (2, 0),
    k_l: (5, 0),
    m: 0,
  };

  if v1.i_j.0!= 2 {
    std::process::exit(1);
  }

  if v1.k_l.0!= 5 {
    std::process::exit(1);
  }

  std::process::exit(0);
}