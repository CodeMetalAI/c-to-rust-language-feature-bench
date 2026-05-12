fn f_int(x: i32) -> i32 {
    x + 1
}

fn f_double(x: f64) -> f64 {
    x + 0.5
}

fn main() {
    let p_int: fn(i32) -> i32 = f_int;
    let p_double: fn(f64) -> f64 = f_double;

    let v1: *const () = unsafe { std::mem::transmute(p_int) };
    let v2: *const () = unsafe { std::mem::transmute(p_double) };

    let p_double2: fn(f64) -> f64 = unsafe { std::mem::transmute(v1) };
    let p_int2: fn(i32) -> i32 = unsafe { std::mem::transmute(v2) };

    let v1b: *const () = unsafe { std::mem::transmute(p_double2) };
    let v2b: *const () = unsafe { std::mem::transmute(p_int2) };

    if v1b != v1 {
        std::process::exit(1);
    }
    if v2b != v2 {
        std::process::exit(2);
    }

    let p_int3: fn(i32) -> i32 = unsafe { std::mem::transmute(v1b) };
    let p_double3: fn(f64) -> f64 = unsafe { std::mem::transmute(v2b) };

    if unsafe { std::mem::transmute::<_, *const ()>(p_int3) } != unsafe { std::mem::transmute(p_int) } {
        std::process::exit(3);
    }
    if unsafe { std::mem::transmute::<_, *const ()>(p_double3) } != unsafe { std::mem::transmute(p_double) } {
        std::process::exit(4);
    }

    std::process::exit(0);
}