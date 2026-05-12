fn main() {
  let mut x: i32 = 17;
  let p: *const i32 = &x;

  let pc: *const i32 = p;
  let pv: *const i32 = p;
  let pcv: *const i32 = p;

  if pc != p {
    panic!("Error: (const void *)pc != (const void *)p");
  }
  if pv != p {
    panic!("Error: (const void *)pv != (const void *)p");
  }
  if pcv != p {
    panic!("Error: (const void *)pcv != (const void *)p");
  }

  if *pc != 17 {
    panic!("Error: *pc != 17");
  }
  if *pv != 17 {
    panic!("Error: *pv != 17");
  }
  if *pcv != 17 {
    panic!("Error: *pcv != 17");
  }

  ()
}