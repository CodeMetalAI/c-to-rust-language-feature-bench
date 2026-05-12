fn f_int(x: i32) -> i32 {
    x + 1
}

fn f_double(x: f64) -> f64 {
    x + 0.5
}

fn main() {
    let p_int: fn(i32) -> i32 = f_int;
    let p_double: fn(f64) -> f64 = f_double;

    let v1: *const () = p_int as *const ();
    let v2: *const () = p_double as *const ();

    let p_double2: fn(f64) -> f64 = v1 as fn(f64) -> f64;
    let p_int2: fn(i32) -> i32 = v2 as fn(i32) -> i32;

    let v1b: *const () = p_double2 as *const ();
    let v2b: *const () = p_int2 as *const ();

    if v1b != v1 {
        std::process::exit(1);
    }
    if v2b != v2 {
        std::process::exit(2);
    }

    let p_int3: fn(i32) -> i32 = v1b as fn(i32) -> i32;
    let p_double3: fn(f64) -> f64 = v2b as fn(f64) -> f64;

    if (p_int3 as *const ()) != (p_int as *const ()) {
        std::process::exit(3);
    }
    if (p_double3 as *const ()) != (p_double as *const ()) {
        std::process::exit(4);
    }

    std::process::exit(0);
}