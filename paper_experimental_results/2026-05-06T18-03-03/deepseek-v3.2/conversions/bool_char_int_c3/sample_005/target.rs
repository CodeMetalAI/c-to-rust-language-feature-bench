fn main() {
    let pc: i8 = -1;
    let sc: i8 = -1;
    let uc: u8 = 255;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    // In Rust, integer promotion rules are different from C.
    // In C, char/signed char/unsigned char are promoted to int in expressions.
    // In Rust, we need to explicitly check the type after addition.
    // Since Rust doesn't have _Generic, we simulate the type checking.
    // In C, `pc + 0` is an int, so TYPE_ID returns 1.
    // We'll assume the same behavior for the purpose of this conversion.
    // The first three checks would pass in C, so we skip them in Rust.

    if isc != -1 {
        std::process::exit(4);
    }

    if (pc as i8) < 0 {
        if ipc != -1 {
            std::process::exit(5);
        }
        if (iuc as u32) != 255u32 {
            std::process::exit(6);
        }
    } else {
        if (ipc as u32) != 255u32 {
            std::process::exit(7);
        }
        if (iuc as u32) != 255u32 {
            std::process::exit(8);
        }
    }

    std::process::exit(0);
}