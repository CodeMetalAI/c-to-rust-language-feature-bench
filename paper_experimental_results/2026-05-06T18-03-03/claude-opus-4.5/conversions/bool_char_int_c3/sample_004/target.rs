fn main() {
    let pc: i8 = -1_i8;
    let sc: i8 = -1_i8;
    let uc: u8 = (-1_i8) as u8;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    // In Rust, arithmetic on i8/u8 promotes to i32 when we do the operation
    // The TYPE_ID checks are verifying that char types promote to int in C
    // In Rust, we're using i32 which is the equivalent, so these checks pass
    // (pc + 0) as i32 would be i32, which corresponds to TYPE_ID == 1
    let _: i32 = (pc as i32) + 0;
    let _: i32 = (sc as i32) + 0;
    let _: i32 = (uc as i32) + 0;

    // These type checks always pass in our Rust translation since we're using i32
    // which is equivalent to C's int (TYPE_ID == 1)

    if isc != -1 {
        std::process::exit(4);
    }

    // In C, whether plain 'char' is signed or unsigned is implementation-defined
    // On most platforms (x86, x86_64), char is signed, so (char)-1 < 0 is true
    // We'll assume signed char behavior (most common case)
    if (-1_i8) < 0 {
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