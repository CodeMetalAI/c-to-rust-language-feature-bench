fn f(x: i32) -> i32 {
    x
}

fn main() {
    let p0: *const i32 = std::ptr::null();
    let v0: *const () = std::ptr::null();
    let p1: *const i32 = v0 as *const i32;

    let fp0: fn(i32) -> i32 = unsafe { std::mem::transmute(std::ptr::null::<fn(i32) -> i32>()) };
    let fp1: fn(i32) -> i32 = unsafe { std::mem::transmute(std::ptr::null::<fn(i32) -> i32>()) };
    let fp2: fn(i32) -> i32 = unsafe { std::mem::transmute(v0) };

    if !p0.eq(&p1) {
        std::process::exit(1);
    }
    if !p0.eq(&std::ptr::null()) {
        std::process::exit(2);
    }

    if !v0.eq(&std::ptr::null()) {
        std::process::exit(3);
    }
    if !v0.eq(&p0 as *const () as *const ()) {
        std::process::exit(4);
    }

    if !fp0.eq(&fp1) {
        std::process::exit(5);
    }
    if !fp1.eq(&fp2) {
        std::process::exit(6);
    }
    if !fp0.eq(&unsafe { std::mem::transmute(std::ptr::null::<fn(i32) -> i32>()) }) {
        std::process::exit(7);
    }

    if !p0.eq(&fp0 as *const fn(i32) -> i32 as *const i32) {
        std::process::exit(8);
    }

    std::process::exit(0);
}