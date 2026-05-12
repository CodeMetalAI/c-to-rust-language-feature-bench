fn main() {
  let pc: i8 = -1;
  let sc: i8 = -1;
  let uc: u8 = 255;

  let ipc: i32 = pc as i32;
  let isc: i32 = sc as i32;
  let iuc: u32 = uc as u32;

  if!(pc + 0).is_i32() {
    return;
  }
  if!(sc + 0).is_i32() {
    return;
  }
  if!(uc + 0).is_i32() {
    return;
  }

  if isc!= -1 {
    return;
  }

  if (-1 < 0) as bool {
    if ipc!= -1 {
      return;
    }
    if iuc!= 255 {
      return;
    }
  } else {
    if ipc as u32!= 255 {
      return;
    }
    if iuc!= 255 {
      return;
    }
  }

  println!("Success");
}