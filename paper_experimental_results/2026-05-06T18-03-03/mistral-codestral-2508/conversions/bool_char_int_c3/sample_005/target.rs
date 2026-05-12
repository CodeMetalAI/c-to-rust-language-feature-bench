fn main() {
    let pc: i8 = -1;
    let sc: i8 = -1;
    let uc: u8 = u8::MAX;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    if isc != -1 {
        std::process::exit(4);
    }

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
}