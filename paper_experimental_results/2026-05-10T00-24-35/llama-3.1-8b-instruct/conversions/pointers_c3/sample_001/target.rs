fn f(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x = 1;
    let px = &x;
    let pf = f;

    let mut n1: *mut i32 = std::ptr::null_mut();
    let v0: *const i32 = std::ptr::null();
    let n2 = unsafe { &*v0 };

    let nf1: unsafe fn(i32) -> i32 = std::mem::forget(std::ptr::null_mut());
    let vf0: *const i32 = std::ptr::null();
    let nf2 = unsafe { std::mem::forget(vf0 as *const _ as unsafe fn(i32) -> i32) };

    if !n1.is_null() {
        println!("n1 != 0");
        return;
    }
    if !n2.is_null() {
        println!("n2 != 0");
        return;
    }
    if !v0.is_null() {
        println!("v0 != 0");
        return;
    }

    if n1 == px {
        println!("n1 == px");
        return;
    }
    if n2 == px {
        println!("n2 == px");
        return;
    }

    if !nf1.is_null() {
        println!("nf1 != 0");
        return;
    }
    if !nf2.is_null() {
        println!("nf2 != 0");
        return;
    }
    if nf1 == pf {
        println!("nf1 == pf");
        return;
    }
    if nf2 == pf {
        println!("nf2 == pf");
        return;
    }

    let vx = px as *const _;
    let vf = pf as *const _;

    if vx.is_null() {
        println!("vx == 0");
        return;
    }
    if vf.is_null() {
        println!("vf == 0");
        return;
    }
    if vx == v0 {
        println!("vx == v0");
        return;
    }
    if vf == vf0 {
        println!("vf == vf0");
        return;
    }

    println!("All checks passed");
    return;
}