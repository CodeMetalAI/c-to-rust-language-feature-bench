typedef unsigned long usize;

extern int *x;
extern int y[];

int y[7];

static int backing[3] = {10, 20, 30};
int *x = backing;

#define OFFSETOF(type, member) ((usize) & (((type *)0)->member))

struct HoldP {
  p: *mut i32,
}

struct HoldA {
  tag: i32,
  a: [i32; 7],
}

static fn sum_ptr(p: *const i32, n: usize) -> i32 {
  let mut s = 0;
  let mut i = 0;
  while i < n {
    s += unsafe { *p.add(i) };
    i += 1;
  }
  s
}

static fn sum_arr7(a: &[i32; 7]) -> i32 {
  let mut s = 0;
  let mut i = 0;
  while i < 7 {
    s += a[i];
    i += 1;
  }
  s
}

static fn mutate_through_pointer(p: &mut [i32; 3]) -> i32 {
  p[1] += 5;
  p[0] + p[1] + p[2]
}

fn main() -> i32 {
  let mut y = [1, 2, 3, 4, 5, 6, 7];

  if sum_arr7(&y) != (1 + 2 + 3 + 4 + 5 + 6 + 7) {
    return 1;
  }

  if sum_ptr(unsafe { x }, 3) != (10 + 20 + 30) {
    return 2;
  }

  let mut backing = [10, 20, 30];
  let mut x = backing.as_mut();

  if mutate_through_pointer(unsafe { &mut *x }) != (10 + 25 + 30) {
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
    struct HoldP {
      p: *const i32,
    }
    struct HoldA {
      tag: i32,
      a: [i32; 7],
    }
    let mut hp = HoldP { p: y.as_ref() };
    if hp.p[0] != 1 {
      return 6;
    }

    let ha = HoldA {
      tag: 0,
      a: y,
    };

    
    if (&ha.a as *const [i32; 7] as *const u8 != (&ha as *const HoldA as *const u8).add(0)) {
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