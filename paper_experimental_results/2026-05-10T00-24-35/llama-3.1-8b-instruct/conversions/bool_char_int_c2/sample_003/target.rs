fn type_id<T: Copy>(x: T) -> i32 {
  match std::any::type_id_of::<T>() {
    std::any::TypeId::Int => 1,
    std::any::TypeId::Uint => 2,
    std::any::TypeId::Float => 7,
    std::any::TypeId::Double => 8,
    std::any::TypeId::LongDouble => 9,
    _ => 99,
  }
}

#[repr(C)]
struct BF {
  u1: u8,
  i1: i8,
  b1: bool,
}

fn main() {
  assert_eq!(type_id((1 as i8) + 0, 1);
  assert_eq!(type_id((1 as u8) + 0, 1);
  assert_eq!(type_id((1 as i16) + 0, 1);
  assert_eq!(type_id((1 as u16) + 0, 1);

  let mut bf = BF {
    u1: 1,
    i1: -1,
    b1: true,
  };

  assert_eq!(type_id(bf.u1 + 0, 1);
  assert_eq!(type_id(bf.i1 + 0, 1);
  assert_eq!(type_id(bf.b1 + 0, 1);

  assert_eq!(type_id(1.0f32, 7);
  assert_eq!(type_id(1.0, 8);
  assert_eq!(type_id(1.0f64, 8);
  assert_eq!(type_id(1.0f80, 9);

  println!("Tests passed");
}