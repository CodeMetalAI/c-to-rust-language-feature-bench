fn f(x: i32) -> i32 {
    x
}

fn main() {
    let p0: *const i32 = std::ptr::null();
    let v0: *const () = std::ptr::null();
    let p1: *const i32 = v0 as *const i32;

    let fp0: fn(i32) -> i32 = None;
    let fp1: fn(i32) -> i32 = Some(f);
    let fp2: fn(i32) -> i32 = fp1;

    if p0 != p1 {
        println!("p0 != p1");
        std::process::exit(1);
    }
    if p0 != std::ptr::null() {
        println!("p0 != std::ptr::null()");
        std::process::exit(2);
    }

    if v0 != std::ptr::null() {
        println!("v0 != std::ptr::null()");
        std::process::exit(3);
    }
    if v0 != p0 as *const () {
        println!("v0 != p0 as *const ()");
        std::process::exit(4);
    }

    if fp0 != fp1 {
        println!("fp0 != fp1");
        std::process::exit(5);
    }
    if fp1 != fp2 {
        println!("fp1 != fp2");
        std::process::exit(6);
    }
    if fp0 != None {
        println!("fp0 != None");
        std::process::exit(7);
    }

    if p0 != fp0 as *const i32 {
        println!("p0 != fp0 as *const i32");
        std::process::exit(8);
    }

    println!("All assertions passed");
    std::process::exit(0);
}