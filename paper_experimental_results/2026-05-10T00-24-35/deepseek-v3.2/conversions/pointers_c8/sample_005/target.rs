fn f_int(x: i32) -> i32 {
    x + 1
}

fn f_double(x: f64) -> f64 {
    x + 0.5
}

fn main() {
    let p_int = f_int;
    let p_double = f_double;

    let v1 = p_int as usize;
    let v2 = p_double as usize;

    let p_double2: fn(f64) -> f64 = unsafe { std::mem::transmute(v1) };
    let p_int2: fn(i32) -> i32 = unsafe { std::mem::transmute(v2) };

    let v1b = p_double2 as usize;
    let v2b = p_int2 as usize;

    if v1b != v1 {
        return std::process::exit(1);
    }
    if v2b != v2 {
        return std::process::exit(2);
    }

    let p_int3: fn(i32) -> i32 = unsafe { std::mem::transmute(v1b) };
    let p_double3: fn(f64) -> f64 = unsafe { std::mem::transmute(v2b) };

    if (p_int3 as usize) != (p_int as usize) {
        return std::process::exit(3);
    }
    if (p_double3 as usize) != (p_double as usize) {
        return std::process::exit(4);
    }

    std::process::exit(0);
}