fn f(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x: i32 = 1;
    let px: *const i32 = &x as *const i32;
    let pf: fn(i32) -> i32 = f;

    let n1: *const i32 = std::ptr::null();
    let v0: usize = 0;
    let n2: *const i32 = std::ptr::null();

    let nf1: Option<fn(i32) -> i32> = None;
    let vf0: usize = 0;
    let nf2: Option<fn(i32) -> i32> = None;

    if n1 != std::ptr::null() {
        std::process::exit(1);
    }
    if n2 != std::ptr::null() {
        std::process::exit(2);
    }
    if v0 != 0 {
        std::process::exit(3);
    }

    if n1 == px {
        std::process::exit(4);
    }
    if n2 == px {
        std::process::exit(5);
    }

    if nf1.is_some() {
        std::process::exit(6);
    }
    if nf2.is_some() {
        std::process::exit(7);
    }
    if nf1 == Some(pf) {
        std::process::exit(8);
    }
    if nf2 == Some(pf) {
        std::process::exit(9);
    }

    let vx: usize = px as usize;
    let vf: usize = pf as usize;

    if vx == 0 {
        std::process::exit(10);
    }
    if vf == 0 {
        std::process::exit(11);
    }
    if vx == v0 {
        std::process::exit(12);
    }
    if vf == vf0 {
        std::process::exit(13);
    }

    std::process::exit(0);
}