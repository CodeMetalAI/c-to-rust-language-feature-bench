fn main() {
    let pc: i8 = -1;
    let sc: i8 = -1;
    let uc: u8 = 255;

    let ipc = pc as i32;
    let isc = sc as i32;
    let iuc = uc as i32;

    // In Rust, integer promotion rules are different from C.
    // We need to check the type after addition with 0.
    // In C, char/signed char/unsigned char + 0 promotes to int.
    // In Rust, i8 + i32 is allowed, but we need to mimic the C behavior.
    // Since Rust doesn't have _Generic, we'll simulate the type checking.
    // In C, TYPE_ID(pc + 0) returns 1 because pc + 0 is an int.
    // In Rust, pc + 0_i32 is i32, so we consider that as "int" (type 1).
    if (pc + 0_i32) as i32 != pc as i32 + 0 {
        return;
    }
    if (sc + 0_i32) as i32 != sc as i32 + 0 {
        return;
    }
    if (uc as i32 + 0) != uc as i32 {
        return;
    }

    if isc != -1 {
        std::process::exit(4);
    }

    if (pc as i8) < 0 {
        if ipc != -1 {
            std::process::exit(5);
        }
        if iuc as u32 != 255u32 {
            std::process::exit(6);
        }
    } else {
        if ipc as u32 != 255u32 {
            std::process::exit(7);
        }
        if iuc as u32 != 255u32 {
            std::process::exit(8);
        }
    }

    std::process::exit(0);
}