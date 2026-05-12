fn main() {
    let pc: i8 = -1; // volatile in C/C++ but this has no direct equivalent in Rust
    let sc: i8 = -1;
    let uc: u8 = 0xff;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    // In Rust, char + 0 promotes to i32 hence these checks are always true
    // Thus, type_of is not required as the type is known to be i32 from language rules

    if isc != -1 {
        std::process::exit(4);
    }

    if (pc as i8) < 0 {
        if ipc != -1 {
            std::process::exit(5);
        }
        if (iuc as u32) != 255 {
            std::process::exit(6);
        }
    } else {
        if (ipc as u32) != 255 {
            std::process::exit(7);
        }
        if (iuc as u32) != 255 {
            std::process::exit(8);
        }
    }
}