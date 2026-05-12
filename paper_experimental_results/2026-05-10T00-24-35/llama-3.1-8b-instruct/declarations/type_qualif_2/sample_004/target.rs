// type_qualif_2.rs

const STRUCT_S: struct {
  mem: i32,
} = STRUCT_S {
  mem: 1,
};

#[derive(Debug, Clone, Copy)]
struct A(#[array] [i32; 2; 3]);

const A_1: A = A([4, 5, 6; 7, 8, 9]);

fn use_int(x: i32) -> i32 {
  x
}

fn main() {
  let struct_s = STRUCT_S {
    mem: STRUCT_S.mem + 10,
  };

  let pci = &STRUCT_S.mem;

  assert_eq!(use_int(struct_s.mem), 11);
  assert_eq!(*pci, 1);

  let a_0_0 = A_1[0][0];
  assert_eq!(a_0_0, 4);
  let a_0_1 = A_1[0][1];
  assert_eq!(a_0_1, 5);
  let a_0_2 = A_1[0][2];
  assert_eq!(a_0_2, 6);
  let a_1_0 = A_1[1][0];
  assert_eq!(a_1_0, 7);
  let a_1_1 = A_1[1][1];
  assert_eq!(a_1_1, 8);
  let a_1_2 = A_1[1][2];
  assert_eq!(a_1_2, 9);
}