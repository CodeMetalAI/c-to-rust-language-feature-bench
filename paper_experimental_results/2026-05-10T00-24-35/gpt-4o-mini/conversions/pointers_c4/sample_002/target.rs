fn f(x: i32) -> i32 {
    x
}

fn main() -> i32 {
    let p0: *const i32 = std::ptr::null();
    let v0: *const () = std::ptr::null();
    let p1: *const i32 = v0 as *const i32;

    let fp0: Option fn(i32) -> i32 = None;
    let fp1: Option fn(i32) -> i32 = None;
    let fp2: Option fn(i32) -> i32 = v0 as Option<fn(i32) -> i32>;

    if p0 != p1 {
        return 1;
    }
    if p0 != std::ptr::null() {
        return 2;
    }

    if v0 != std::ptr::null() {
        return 3;
    }
    if v0 != p0 as *const () {
        return 4;
    }

    if fp0 != fp1 {
        return 5;
    }
    if fp1 != fp2 {
        return 6;
    }
    if fp0.is_some() {
        return 7;
    }

    if p0 != fp0.map(|f| f as *const i32).unwrap_or(std::ptr::null()) {
        return 8;
    }

    return 0;
}