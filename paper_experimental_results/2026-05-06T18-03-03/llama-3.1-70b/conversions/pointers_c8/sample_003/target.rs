fn f_int(x: i32) -> i32 { x + 1 }
fn f_double(x: f64) -> f64 { x + 0.5 }

fn main() {
    let p_int: fn(i32) -> i32 = f_int;
    let p_double: fn(f64) -> f64 = f_double;

    let v1: *const () = p_int as *const ();
    let v2: *const () = p_double as *const ();

    let p_double2: fn(f64) -> f64 = unsafe { std::mem::transmute(v1) };
    let p_int2: fn(i32) -> i32 = unsafe { std::mem::transmute(v2) };

    let v1b: *const () = p_double2 as *const ();
    let v2b: *const () = p_int2 as *const ();

    if v1b as usize!= v1 as usize {
        std::process::exit(1);
    }
    if v2b as usize!= v2 as usize {
        std::process::exit(2);
    }

    let p_int3: fn(i32) -> i32 = unsafe { std::mem::transmute(v1b) };
    let p_double3: fn(f64) -> f64 = unsafe { std::mem::transmute(v2b) };

    if p_int3 as usize!= p_int as usize {
        std::process::exit(3);
    }
    if p_double3 as usize!= p_double as usize {
        std::process::exit(4);
    }
}