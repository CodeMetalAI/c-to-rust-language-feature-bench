fn main() {
  let mut a = 1;
  let mut b = 2;
  let mut c = 3;
  let mut d = 4;
  if (a < b) == (c < d) && (a < b) == true {
    a = 2;
    b = 1;
    c = 4;
    d = 3;
    if (a < b) == (c < d) && (a < b) == true {
      a = 1;
      b = 2;
      c = 4;
      d = 3;
      if (a < b) == (c < d) && (a < b) == false {
        a = 2;
        b = 1;
        c = 3;
        d = 4;
        if (a < b) == (c < d) && (a < b) == false {
          a = 1;
          b = 2;
          c = 3;
          d = 4;
          if (a < b) == (c < d) && (a < b) == true {
            return;
          } else {
            std::process::exit(5);
          }
        } else {
          std::process::exit(4);
        }
      } else {
        std::process::exit(3);
      }
    } else {
      std::process::exit(2);
    }
  } else {
    std::process::exit(1);
  }
}