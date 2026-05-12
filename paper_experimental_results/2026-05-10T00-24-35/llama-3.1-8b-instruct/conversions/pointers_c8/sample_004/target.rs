fn f_int(x: i32) -> i32 {
    x + 1
}

fn f_double(x: f64) -> f64 {
    x + 0.5
}

fn main() {
    let p_int: fn(i32) -> i32 = f_int;
    let p_double: fn(f64) -> f64 = f_double;

    let v1: *const () = &p_int as *const ();
    let v2: *const () = &p_double as *const ();

    let p_double2: fn(f64) -> f64 = std::mem::transmute(v1);
    let p_int2: fn(i32) -> i32 = std::mem::transmute(v2);

    let v1b: *const () = &p_double2 as *const ();
    let v2b: *const () = &p_int2 as *const ();

    if v1b != &p_int as *const () {
        panic!("v1b != p_int");
    }
    if v2b != &p_double as *const () {
        panic!("v2b != p_double");
    }

    let p_int3: fn(i32) -> i32 = std::mem::transmute(v1b);
    let p_double3: fn(f64) -> f64 = std::mem::transmute(v2b);

    if &p_int3 as *const () != &p_int as *const () {
        panic!("p_int3 != p_int");
    }
    if &p_double3 as *const () != &p_double as *const () {
        panic!("p_double3 != p_double");
    }

    println!("All tests passed");
    std::process::exit(0);
}