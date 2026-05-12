fn main() {
    let pc: i8 = -1;
    let sc: i8 = -1;
    let uc: u8 = 255;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    // TYPE_ID checks are always true since pc + 0, etc., result in i32

    if isc != -1 {
        std::process::exit(4);
    }

    let char_minus_one: i8 = -1;
    if char_minus_one < 0 {
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

    std::process::exit(0);
}