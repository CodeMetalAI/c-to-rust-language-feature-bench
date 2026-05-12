fn main() {
    fn f_int(x: i32) -> i32 { x + 1 }
    fn f_double(x: f64) -> f64 { x + 0.5 }

    let p_int: fn(i32) -> i32 = f_int;
    let p_double: fn(f64) -> f64 = f_double;

    let v1 = p_int as *const ();
    let v2 = p_double as *const ();

    // Simulate casting from void* to specific function pointer types
    let p_double2: fn(f64) -> f64 = unsafe { std::mem::transmute(v1) };
    let p_int2: fn(i32) -> i32 = unsafe { std::mem::transmute(v2) };

    let v1b = p_double2 as *const ();
    let v2b = p_int2 as *const ();

    if v1b != v1 {
        std::process::exit(1);
    }
    if v2b != v2 {
        std::process::exit(2);
    }

    // Simulate casting back from void* to the original function pointer types
    let p_int3: fn(i32) -> i32 = unsafe { std::mem::transmute(v1b) };
    let p_double3: fn(f64) -> f64 = unsafe { std::mem::transmute(v2b) };

    if p_int3 as *const () != p_int as *const () {
        std::process::exit(3);
    }
    if p_double3 as *const () != p_double as *const () {
        std::process::exit(4);
    }
}