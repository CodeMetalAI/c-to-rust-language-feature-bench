fn takes_t1(v: S1) -> i32 { v.x + 1 }
fn takes_s1(v: S1) -> i32 { v.x + 2 }
fn takes_tp1(p: &mut S1) -> i32 { 
  p.x += 3; 
  p.x 
}
fn takes_int(v: i32) -> i32 { v + 4 }
fn takes_t2(v: S2) -> i32 { v.x + 5 }

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct S1 { x: i32 }
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct S2 { x: i32 }

fn main() {
  let mut a = S1 { x: 10 };
  let mut b = S1 { x: 20 };

  if takes_t1(a)!= 11 { return; }
  if takes_s1(a)!= 12 { return; }
  if takes_t1(b)!= 21 { return; }
  if takes_s1(b)!= 22 { return; }

  let p = &mut a;
  if takes_tp1(p)!= 13 { return; }
  if a.x!= 13 { return; }
  if takes_int(a.x)!= 17 { return; }

  let q: usize = std::mem::size_of::<S1>() + std::mem::size_of::<S1>();
  if q == 0 { return; }

  let mut c = S2 { x: 30 };
  let r = &mut c;
  if r.x!= 30 { return; }
  if takes_t2(c)!= 35 { return; }

  println!("Success");
}