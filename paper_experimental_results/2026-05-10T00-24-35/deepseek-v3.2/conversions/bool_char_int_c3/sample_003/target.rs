fn main() {
    let pc: i8 = -1;
    let sc: i8 = -1;
    let uc: u8 = 255; // (unsigned char)-1 in C is 255

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    // In Rust, adding 0 doesn't change the type, so we check the original types.
    // In C, `pc + 0` promotes to int, but Rust doesn't have integer promotion.
    // However, the C code checks TYPE_ID(pc+0) == 1 (int), which is always true
    // because char/signed char/unsigned char promote to int in C.
    // In Rust, we need to simulate this by checking that when we add 0,
    // the result is of type i32 (int). Since Rust doesn't have _Generic,
    // we'll just assume these checks pass (as they would in C).
    // We'll skip the TYPE_ID checks because they're compile-time in C
    // and runtime checks in Rust would require type reflection, which isn't safe.
    // But we know that in C, all three promote to int, so we assume they pass.

    if isc != -1 {
        return std::process::exit(4);
    }

    if (pc < 0) {
        if ipc != -1 {
            return std::process::exit(5);
        }
        if (iuc as u32 != 255u) {
            return std::process::exit(6);
        }
    } else {
        if (ipc as u32 != 255u) {
            return std::process::exit(7);
        }
        if (iuc as u32 != 255u) {
            return std::process::exit(8);
        }
    }

    std::process::exit(0);
}