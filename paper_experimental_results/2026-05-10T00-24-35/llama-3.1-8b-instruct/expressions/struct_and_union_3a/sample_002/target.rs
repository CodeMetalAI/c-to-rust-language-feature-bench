// Define a union
#[repr(C)]
union U {
  struct {
    pub alltypes: i32,
  } n,
  struct {
    pub type_: i32,
    pub intnode: i32,
  } ni,
  struct {
    pub type_: i32,
    pub doublenode: f64,
  } nf,
}

fn main() {
  // Create a new instance of the union
  let mut u = U { nf: U { n: U { type_: 1, alltypes: 0 } } };

  // Access members of the union
  u.nf.type_ = 1;

  u.nf.doublenode = 3.14;

  // Check the values
  if u.n.alltypes != 1 {
    return 1;
  }
  if u.ni.type_ != 1 {
    return 1;
  }
  if u.nf.type_ != 1 {
    return 2;
  }
  if u.nf.doublenode != 3.14 {
    return 3;
  }

  // Return success
  return 0;
}