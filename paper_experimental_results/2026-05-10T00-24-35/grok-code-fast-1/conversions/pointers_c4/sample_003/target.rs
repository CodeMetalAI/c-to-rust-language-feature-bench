fn f(x: i32) -> i32 { x }

fn main() -> i32 {
    let p0: *const i32 = std::ptr::null();
    let v0: *const () = std::ptr::null();
    let p1: *const i32 = v0 as *const i32;

    let fp0: *const () = std::ptr::null();
    let fp1: *const () = std::ptr::null();
    let fp2: *const () = v0;

    if p0 != p1 {
        return 1;
    }
    if p0 != std::ptr::null::<i32>() {
        return 2;
    }

    if v0 != std::ptr::null::<()>() {
        return 3;
    }
    if v0 != (p0 as *const ()) {
        return 4;
    }

    if fp0 != fp1 {
        return 5;
    }
    if fp1 != fp2 {
        return 6;
    }
    if fp0 != std::ptr::null::<()>() {
        return 7;
    }

    if (p0 as *const ()) != fp0 {
        return 8;
    }

    0
}