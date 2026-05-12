fn f_int(x: i32) -> i32 {
    x + 1
}

fn f_double(x: f64) -> f64 {
    x + 0.5
}

fn main() {
    let p_int: fn(i32) -> i32 = f_int;
    let p_double: fn(f64) -> f64 = f_double;

    let v1: *const () = &p_int as *const _;
    let v2: *const () = &p_double as *const _;

    let p_double2: fn(f64) -> f64 = unsafe { &*(v1 as *const fn(i32) -> i32) };
    let p_int2: fn(i32) -> i32 = unsafe { &*(v2 as *const fn(f64) -> f64) };

    let v1b: *const () = &p_double2 as *const _;
    let v2b: *const () = &p_int2 as *const _;

    if v1b!= v1 {
        panic!("v1b!= v1");
    }
    if v2b!= v2 {
        panic!("v2b!= v2");
    }

    let p_int3: fn(i32) -> i32 = unsafe { &*(v1b as *const fn(f64) -> f64) };
    let p_double3: fn(f64) -> f64 = unsafe { &*(v2b as *const fn(i32) -> i32) };

    if v1b as *const fn(i32) -> i32!= v1 as *const fn(i32) -> i32 {
        panic!("v1b as *const fn(i32) -> i32!= v1 as *const fn(i32) -> i32");
    }
    if v2b as *const fn(f64) -> f64!= v2 as *const fn(f64) -> f64 {
        panic!("v2b as *const fn(f64) -> f64!= v2 as *const fn(f64) -> f64");
    }
}