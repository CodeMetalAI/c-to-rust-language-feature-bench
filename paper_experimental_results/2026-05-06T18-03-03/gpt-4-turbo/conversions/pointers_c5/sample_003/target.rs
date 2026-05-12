fn main() {
    let x = 1;
    let px = &x as *const i32 as usize; // equivalent to uintptr_t in C

    let p2 = px as *const i32;

    if p2 as usize != px {
        std::process::exit(1);
    }

    let z = 1usize;
    let vp = z as *const (); // equivalent to void* in C
    let z2 = vp as usize;

    if z2 != z {
        std::process::exit(2);
    }
}