fn f(x: i32) -> i32 {
    x
}

fn main() {
    let mut p0: *const i32 = std::ptr::null();
    let mut v0: *const () = std::ptr::null();

    let p1: *const i32 = v0 as *const i32;

    let fp0: Option<fn(i32) -> i32> = None;
    let fp1: Option<fn(i32) -> i32> = fp0;
    let fp2: Option<fn(i32) -> i32> = v0 as Option<fn(i32) -> i32>;

    if p0 != p1 {
        println!("p0 != p1");
        return;
    }
    if p0 != std::ptr::null() {
        println!("p0 != std::ptr::null()");
        return;
    }

    if v0 != std::ptr::null() {
        println!("v0 != std::ptr::null()");
        return;
    }
    if v0 != p0 as *const () {
        println!("v0 != p0 as *const ()");
        return;
    }

    if fp0 != fp1 {
        println!("fp0 != fp1");
        return;
    }
    if fp1 != fp2 {
        println!("fp1 != fp2");
        return;
    }
    if fp0 != None {
        println!("fp0 != None");
        return;
    }

    if p0 != fp0 as *const i32 {
        println!("p0 != fp0 as *const i32");
        return;
    }

    println!("All assertions passed");
}