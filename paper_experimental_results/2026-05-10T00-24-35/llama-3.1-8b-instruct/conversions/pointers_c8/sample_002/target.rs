fn f_int(x: i32) -> i32 {
    x + 1
}

fn f_double(x: f64) -> f64 {
    x + 0.5
}

fn main() {
    let p_int: fn(i32) -> i32 = f_int;
    let p_double: fn(f64) -> f64 = f_double;

    let v1: *const dyn Fn(i32) -> i32 = &p_int;
    let v2: *const dyn Fn(f64) -> f64 = &p_double;

    let p_double2 = std::mem::transmute::<*, fn(f64) -> f64>(v1);
    let p_int2 = std::mem::transmute::<*, fn(i32) -> i32>(v2);

    let v1b: *const dyn Fn(f64) -> f64 = &p_double2;
    let v2b: *const dyn Fn(i32) -> i32 = &p_int2;

    if v1b != v1 {
        println!("Test 1 failed");
    }
    if v2b != v2 {
        println!("Test 2 failed");
    }

    let p_int3 = std::mem::transmute::<*, fn(i32) -> i32>(v1b);
    let p_double3 = std::mem::transmute::<*, fn(f64) -> f64>(v2b);

    if p_int3 != p_int {
        println!("Test 3 failed");
    }
    if p_double3 != p_double {
        println!("Test 4 failed");
    }
}