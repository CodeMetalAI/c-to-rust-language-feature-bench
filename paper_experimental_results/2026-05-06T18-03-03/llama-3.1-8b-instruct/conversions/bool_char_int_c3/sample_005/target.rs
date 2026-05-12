fn main() {
    let pc: i8 = -1;
    let sc: i8 = -1;
    let uc: u8 = 255;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: u32 = uc as u32;

    if ipc!= -1 {
        return 5;
    }
    if iuc!= 255 {
        return 6;
    }

    if pc < 0 {
        if ipc!= -1 {
            return 7;
        }
        if iuc!= 255 {
            return 8;
        }
    } else {
        if ipc!= 255 {
            return 7;
        }
        if iuc!= 255 {
            return 8;
        }
    }

    return 0;
}