fn f(x: i32) -> i32 {
    x + 1
}

fn main() {
    unsafe {
        let x = 1;
        let px = &x as *const i32;
        let pf = f as *const ();
        let n1: *const i32 = std::ptr::null();
        let v0: *const () = std::ptr::null();
        let n2: *const i32 = v0 as *const i32;
        let nf1: *const () = std::ptr::null();
        let vf0: *const () = std::ptr::null();
        let nf2: *const () = vf0 as *const ();
        if n1 != std::ptr::null() {
            std::process::exit(1);
        }
        if n2 != std::ptr::null() {
            std::process::exit(2);
        }
        if v0 != std::ptr::null() {
            std::process::exit(3);
        }
        if n1 == px {
            std::process::exit(4);
        }
        if n2 == px {
            std::process::exit(5);
        }
        if nf1 != std::ptr::null() {
            std::process::exit(6);
        }
        if nf2 != std::ptr::null() {
            std::process::exit(7);
        }
        if nf1 == pf {
            std::process::exit(8);
        }
        if nf2 == pf {
            std::process::exit(9);
        }
        let vx = px as *const ();
        let vf = pf;
        if vx == std::ptr::null() {
            std::process::exit(10);
        }
        if vf == std::ptr::null() {
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
}