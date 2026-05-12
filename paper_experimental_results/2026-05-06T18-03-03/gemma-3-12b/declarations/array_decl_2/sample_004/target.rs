typedef unsigned long usize;

extern int *x;
extern int y[];

int y[7];

static int backing[3] = {10, 20, 30};
int *x = backing;

#define OFFSETOF(type, member) ((usize) & (((type *)0)->member))

struct HoldP {
  p: *const i32,
}

struct HoldA {
  tag: i32,
  a: [i32; 7],
}

fn sum_ptr(p: *const i32, n: usize) -> i32 {
  let mut s = 0;
  for i in 0..n {
    s += unsafe { *p.add(i) };
  }
  s
}

fn sum_arr7(a: &[i32; 7]) -> i32 {
  let mut s = 0;
  for &val in a {
    s += val;
  }
  s
}

fn mutate_through_pointer(p: &mut [i32]) -> i32 {
  unsafe {
    p[1] += 5;
  }
  p[0] + p[1] + p[2]
}

fn main() {
  let mut y = [1, 2, 3, 4, 5, 6, 7];

  if sum_arr7(&y) != (1 + 2 + 3 + 4 + 5 + 6 + 7) as i32 {
    return 1;
  }

  if sum_ptr(x as *const i32, 3) != (10 + 20 + 30) as i32 {
    return 2;
  }

  if mutate_through_pointer(&mut y) != (10 + 25 + 30) as i32 {
    return 3;
  }

  if backing[1] != 25 {
    return 4;
  }

  {
    let t = &y[..];
    if t[6] != 7 {
      return 5;
    }
  }

  {
    let hp = HoldP { p: y.as_ptr() };
    let ha = unsafe { &*(y.as_ptr() as *mut HoldA) };

    if hp.p[0] != 1 {
      return 6;
    }

    if (y.as_ptr() as usize) + std::mem::size_of::<i32>() != (ha as *const HoldA as usize) + std::mem::size_of::<i32>() {
      return 7;
    }

    if ha.a[2] != 3 {
      return 8;
    }
  }

  if y[0] != 1 {
    return 9;
  }

  0
}