// initialization_12.rs
const MAX: usize = 12;

fn main() -> i32 {
  let mut a = [1, 3, 5, 7, 9, 8, 6, 4, 2, 0];

  if a[0] != 1 {
    1
  } else if a[1] != 3 {
    2
  } else if a[2] != 5 {
    3
  } else if a[3] != 7 {
    4
  } else if a[4] != 9 {
    5
  } else if a[MAX - 5] != 8 {
    6
  } else if a[MAX - 4] != 6 {
    7
  } else if a[MAX - 3] != 4 {
    8
  } else if a[MAX - 2] != 2 {
    9
  } else if a[MAX - 1] != 0 {
    10
  } else {
    let mut i = 5;
    while i < MAX - 5 {
      if a[i] != 0 {
        11
      }
      i += 1;
    }
    0
  }
}