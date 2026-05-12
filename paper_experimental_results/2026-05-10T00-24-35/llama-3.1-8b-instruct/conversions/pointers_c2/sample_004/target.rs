fn main() {
  let x = 17;
  let mut p: *const i32 = &x;

  let pc: *const i32 = p;
  let pv: *const i32 = p;

  if pc != p {
    panic!("Pointer comparison failed");
  }
  if pv != p {
    panic!("Pointer comparison failed");
  }

  if *pc != 17 {
    panic!(" Dereference comparison failed");
  }
  if *pv != 17 {
    panic!("Dereference comparison failed");
  }

  println!("Main function finished.");
}