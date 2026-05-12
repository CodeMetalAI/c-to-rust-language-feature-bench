fn f(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x: i32 = 1;
    let px: *const i32 = &x;
    let pf: fn(i32) -> i32 = f;

    let n1: *const i32 = 0 as *const i32;
    let v0: *const () = 0 as *const ();
    let n2: *const i32 = v0 as *const i32;

    let nf1: *const () = 0 as *const ();
    let vf0: *const () = 0 as *const ();
    let nf2: *const () = vf0 as *const ();

    if n1 != (0 as *const i32) {
        std::process::exit(1);
    }
    if n2 != (0 as *const i32) {
        std::process::exit(2);
    }
    if v0 != (0 as *const ()) {
        std::process::exit(3);
    }

    if n1 == px {
        std::process::exit(4);
    }
    if n2 == px {
        std::process::exit(5);
    }

    if nf1 != (0 as *const ()) {
        std::process::exit(6);
    }
    if nf2 != (0 as *const ()) {
        std::process::exit(7);
    }
    if nf1 == (pf as *const ()) {
        std::process::exit(8);
    }
    if nf2 == (pf as *const ()) {
        std::process::exit(9);
    }

    let vx: *const () = px as *const ();
    let vf: *const () = pf as *const ();

    if vx == (0 as *const ()) {
        std::process::exit(10);
    }
    if vf == (0 as *const ()) {
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