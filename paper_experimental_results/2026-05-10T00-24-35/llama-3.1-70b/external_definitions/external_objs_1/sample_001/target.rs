static mut i1: i32 = 1;
static mut i2: i32 = 2;
static mut i3: i32 = 3;

static mut i4: i32 = 0;
static mut i5: i32 = 0;

static mut p_i1: *mut i32 = &mut i1;
static mut p_i2: *mut i32 = &mut i2;
static mut p_i4: *mut i32 = &mut i4;
static mut p_i5: *mut i32 = &mut i5;

fn main() {
  if unsafe { i1 } != 1 {
    return 1;
  }
  if unsafe { i2 } != 2 {
    return 2;
  }
  if unsafe { i3 } != 3 {
    return 3;
  }

  if unsafe { i4 } != 0 {
    return 4;
  }
  if unsafe { i5 } != 0 {
    return 5;
  }

  if unsafe { p_i1 } != &mut i1 as *mut i32 {
    return 6;
  }
  if unsafe { p_i2 } != &mut i2 as *mut i32 {
    return 7;
  }
  if unsafe { p_i4 } != &mut i4 as *mut i32 {
    return 8;
  }
  if unsafe { p_i5 } != &mut i5 as *mut i32 {
    return 9;
  }

  unsafe {
    i1 = 10;
    i2 = 20;
    i4 = 30;
    i5 = 40;
  }

  if unsafe { *p_i1 } != 10 {
    return 10;
  }
  if unsafe { *p_i2 } != 20 {
    return 11;
  }
  if unsafe { *p_i4 } != 30 {
    return 12;
  }
  if unsafe { *p_i5 } != 40 {
    return 13;
  }

  std::process::exit(0);
}