fn f_int(x: i32) -> i32 { x + 1 }
fn f_double(x: f64) -> f64 { x + 0.5 }

fn main() {
  let p_int = f_int as fn(i32) -> i32;
  let p_double = f_double as fn(f64) -> f64;

  let v1 = p_int as *const ();
  let v2 = p_double as *const ();

  let p_double2 = v1 as fn(f64) -> f64;
  let p_int2 = v2 as fn(i32) -> i32;

  let v1b = p_double2 as *const ();
  let v2b = p_int2 as *const ();

  if v1b as usize!= v1 as usize {
    return;
  }
  if v2b as usize!= v2 as usize {
    std::process::exit(2);
  }

  let p_int3 = v1b as fn(i32) -> i32;
  let p_double3 = v2b as fn(f64) -> f64;

  if p_int3 as *const () as usize!= p_int as *const () as usize {
    std::process::exit(3);
  }
  if p_double3 as *const () as usize!= p_double as *const () as usize {
    std::process::exit(4);
  }
}