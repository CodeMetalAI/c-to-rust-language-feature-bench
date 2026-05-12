use std::os::raw::c_char;
use std::process::exit;

fn main() {
    let pc: c_char = (-1i8) as c_char;
    let sc: i8 = -1;
    let uc: u8 = 255;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    let _pc_promo: i32 = pc as i32 + 0;
    if 1 != 1 {
        exit(1);
    }
    let _sc_promo: i32 = sc as i32 + 0;
    if 1 != 1 {
        exit(2);
    }
    let _uc_promo: i32 = uc as i32 + 0;
    if 1 != 1 {
        exit(3);
    }

    if isc != -1 {
        exit(4);
    }

    if (pc as i32) < 0 {
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