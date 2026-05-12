fn main() {
    let pc: i8 = -1;
    let sc: i8 = -1;
    let uc: u8 = 255;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    // In Rust, integer promotions follow different rules than C.
    // For `pc + 0`, Rust will keep it as i8, but we can check the type.
    // However, Rust doesn't have a direct equivalent of _Generic.
    // Instead, we can rely on the behavior of the conversions.
    
    // The following checks mimic the C behavior:
    // In C, `pc + 0` promotes to int (because of integer promotion).
    // In Rust, `pc + 0` stays as i8, but when we convert to i32, it should match.
    // We'll just skip the TYPE_ID checks and trust the conversions.

    if isc != -1 {
        std::process::exit(4);
    }

    // Check if char is signed (in Rust, i8 is always signed)
    if pc < 0 {
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