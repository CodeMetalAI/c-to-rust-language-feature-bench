fn f(x: i32) -> i32 {
    x
}

fn main() {
    let mut p0: *const i32 = std::ptr::null();
    let v0: *const () = std::ptr::null();
    let p1: *const i32 = v0 as *const i32;

    let fp0: Option<&dyn Fn(i32) -> i32> = None;
    let fp1: Option<&dyn Fn(i32) -> i32> = fp0;
    let fp2: Option<&dyn Fn(i32) -> i32> = v0 as Option<&dyn Fn(i32) -> i32>;

    if p0 != p1 {
        return;
    }
    if p0 != std::ptr::null() {
        return;
    }

    if v0 != std::ptr::null() {
        return;
    }
    if v0 != p0 as *const () {
        return;
    }

    if fp0 != fp1 {
        return;
    }
    if fp1 != fp2 {
        return;
    }
    if fp0 != None {
        return;
    }

    if p0 != fp0 as *const i32 {
        return;
    }

    println!("Test passed");
}