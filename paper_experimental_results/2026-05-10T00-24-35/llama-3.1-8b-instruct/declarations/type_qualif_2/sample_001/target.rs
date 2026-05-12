const struct s {
  mem: i32,
}

cs = struct s { mem: 1 };

struct s ncs;

const a: [[i32; 3]; 2] = [[4, 5, 6], [7, 8, 9]];

fn use_int(x: i32) -> i32 {
  x
}

fn main() {
  let mut pi = &mut ncs.mem;
  let a = &cs.mem;

  ncs = cs;

  *pi += 10;

  if use_int(ncs.mem)!= 11 {
    return 1;
  }

  if a!= 1 {
    return 2;
  }

  if a[0][0]!= 4 {
    return 3;
  }
  if a[0][1]!= 5 {
    return 4;
  }
  if a[0][2]!= 6 {
    return 5;
  }
  if a[1][0]!= 7 {
    return 6;
  }
  if a[1][1]!= 8 {
    return 7;
  }
  if a[1][2]!= 9 {
    return 8;
  }

  return 0;
}