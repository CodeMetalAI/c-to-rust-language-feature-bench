fn main() {
  struct S {
    n: i32,
    d: [f64; 8],
  };

  struct SS {
    n: i32,
  };

  assert!(std::mem::size_of::<S>() >= std::mem::size_of::<SS>());
  assert!(std::mem::offset_of::<S, _>(std::any::type_name::<S>().split('`').next().unwrap().split(':').last().unwrap()).unwrap() == std::mem::size_of::<S>());

  let mut s1 = S { n: 0, d: [0.0; 8] };
  let mut s2 = S { n: 0, d: [0.0; 8] };

  s1.d[0] = 42.0;
  s2.d[0] = 24.0;

  assert!(s1.d[0] == 42.0);
  assert!(s2.d[0] == 24.0);
}