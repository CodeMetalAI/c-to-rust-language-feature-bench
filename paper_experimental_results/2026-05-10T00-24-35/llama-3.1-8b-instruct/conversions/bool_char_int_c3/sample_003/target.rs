fn main() {
  let pc = -1 as i8;
  let sc = -1 as i8;
  let uc = -1 as u8;

  let ipc: i32 = pc as i32;
  let isc: i32 = sc as i32;
  let iuc: u32 = uc as u32;

  if TYPE_ID(pc + 0)!= 1 {
    return 1;
  }
  if TYPE_ID(sc + 0)!= 1 {
    return 2;
  }
  if TYPE_ID(uc + 0)!= 1 {
    return 3;
  }

  if isc!= -1 {
    return 4;
  }

  if -1 as i8 < 0 {
    if ipc!= -1 {
      return 5;
    }
    if iuc!= 255 {
      return 6;
    }
  } else {
    if i32::from(ipc)!= 255 {
      return 7;
    }
    if iuc!= 255 {
      return 8;
    }
  }

  return 0;
}

fn TYPE_ID<T: Copy + Eq>(x: T) -> u8 {
  match x {
    _ if x == -1 && std::mem::size_of::<T>() == 1 => 1,
    _ if x == 255 => 2,
    _ => 99,
  }
}