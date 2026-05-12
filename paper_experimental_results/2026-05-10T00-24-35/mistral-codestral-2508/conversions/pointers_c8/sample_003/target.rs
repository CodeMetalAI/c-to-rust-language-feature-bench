fn f_int(x: i32) -> i32 {
    x + 1
}

fn f_double(x: f64) -> f64 {
    x + 0.5
}

fn main() {
    let p_int: fn(i32) -> i32 = f_int;
    let p_double: fn(f64) -> f64 = f_double;

    let v1 = &p_int as *const fn(i32) -> i32 as *const ();
    let v2 = &p_double as *const fn(f64) -> f64 as *const ();

    let p_double2: fn(f64) -> f64 = unsafe { std::mem::transmute(v1) };
    let p_int2: fn(i32) -> i32 = unsafe { std::mem::transmute(v2) };

    let v1b = &p_double2 as *const fn(f64) -> f64 as *const ();
    let v2b = &p_int2 as *const fn(i32) -> i32 as *const ();

    if v1b != v1 {
        std::process::exit(1);
    }
    if v2b != v2 {
        std::process::exit(2);
    }

    let p_int3: fn(i32) -> i32 = unsafe { std::mem::transmute(v1b) };
    let p_double3: fn(f64) -> f64 = unsafe { std::mem::transmute(v2b) };

    if &p_int3 as *const fn(i32) -> i32 != &p_int as *const fn(i32) -> i32 {
        std::process::exit(3);
    }
    if &p_double3 as *const fn(f64) -> f64 != &p_double as *const fn(f64) -> f64 {
        std::process::exit(4);
    }

    std::process::exit(0);
}