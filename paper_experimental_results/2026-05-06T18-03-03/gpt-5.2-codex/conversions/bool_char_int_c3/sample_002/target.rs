use std::os::raw::c_char;
use std::process::exit;

fn main() {
    let pc: c_char = (-1i8) as c_char;
    let sc: i8 = -1;
    let uc: u8 = 255;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    // TYPE_ID checks in C always yield int for these expressions; no action needed.

    if isc != -1 {
        exit(4);
    }

    let char_is_signed = (-1i8 as c_char) < (0 as c_char);

    if char_is_signed {
        if ipc != -1 {
            exit(5);
        }
        if (iuc as u32) != 255u32 {
            exit(6);
        }
    } else {
        if (ipc as u32) != 255u32 {
            exit(7);
        }
        if (iuc as u32) != 255u32 {
            exit(8);
        }
    }

    exit(0);
}