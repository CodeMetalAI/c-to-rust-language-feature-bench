fn f(init: i32) -> i32 {
  let mut x = init;
  let p = &mut x as *mut i32;

  unsafe {
    let p_pointer = p as *mut [i32];
    let array = Box::from_raw(p_pointer as *mut [i32]);
    let arr: &[i32] = std::slice::from_raw_parts(array.as_ptr(), 1);
    let mut arr = arr.as_mut_ptr();
    arr.copy_from_slice(&[0]);
    let new_p = Box::into_raw(Box::new([*arr, 0]));
    let new_p = std::ptr::read(new_p as *const i32);
    let p = Box::from_raw(new_p as *mut i32);
  }

  if *p != init {
      1
  } else if *unsafe { p.offset(1) } != 0 {
      2
  } else {
      0
  }
}

fn main() -> i32 {
  if f(7) != 0 {
      10
  } else if f(-3) != 0 {
      11
  } else {
      0
  }
}