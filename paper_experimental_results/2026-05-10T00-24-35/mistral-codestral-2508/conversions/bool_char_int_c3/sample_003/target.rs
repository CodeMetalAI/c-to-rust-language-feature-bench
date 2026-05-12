fn main() {
    let pc: i8 = -1;
    let sc: i8 = -1;
    let uc: u8 = 255;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: u32 = uc as u32;

    if isc != -1 {
        std::process::exit(4);
    }

    if pc < 0 {
        if ipc != -1 {
            std::process::exit(5);
        }
        if iuc != 255 {
            std::process::exit(6);
        }
    } else {
        if ipc as u32 != 255 {
            std::process::exit(7);
        }
        if iuc != 255 {
            std::process::exit(8);
        }
    }
}