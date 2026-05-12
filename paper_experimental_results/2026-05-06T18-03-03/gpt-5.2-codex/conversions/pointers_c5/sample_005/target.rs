fn main() {
    type uintptr_t = usize;

    let x: i32 = 1;
    let px: *const i32 = &x;

    let ip: uintptr_t = px as uintptr_t;
    let p2: *const i32 = ip as *const i32;

    if (p2 as uintptr_t) != ip {
        std::process::exit(1);
    }

    let z: uintptr_t = 1;
    let vp: *const () = z as *const ();
    let z2: uintptr_t = vp as uintptr_t;

    if z2 != z {
        std::process::exit(2);
    }

    std::process::exit(0);
}