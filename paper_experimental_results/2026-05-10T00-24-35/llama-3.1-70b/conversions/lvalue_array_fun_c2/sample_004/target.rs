fn main() {
  let x: i32 = 3;
  let cx: i32 = 4;
  let ax: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(5);

  if std::mem::size_of_val(&x)!= std::mem::size_of::<i32>() {
    return;
  }
  if std::mem::align_of_val(&x)!= std::mem::align_of::<i32>() {
    return;
  }

  if *(&x)!= 3 {
    return;
  }

  let type_id_x = std::any::TypeId::of::<i32>();
  let type_id_ax = std::any::TypeId::of::<std::sync::atomic::AtomicI32>();
  if type_id_x!= std::any::TypeId::of::<i32>() {
    return;
  }
  if type_id_ax!= std::any::TypeId::of::<std::sync::atomic::AtomicI32>() {
    return;
  }

  if std::any::TypeId::of::<i32>()!= std::any::TypeId::of::<i32>() {
    return;
  }
  if std::any::TypeId::of::<i32>()!= std::any::TypeId::of::<i32>() {
    return;
  }

  let y = x;
  if y!= 3 {
    return;
  }

  x += 1;
  if x!= 4 {
    return;
  }

  #[derive(Debug)]
  struct S {
    m: i32,
  }
  let mut s = S { m: 7 };
  if s.m!= 7 {
    return;
  }
  s.m = 8;
  if s.m!= 8 {
    return;
  }

  std::process::exit(0);
}