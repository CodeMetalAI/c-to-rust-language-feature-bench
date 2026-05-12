fn main() {
    let pc = -1 as i8;
    let sc = -1 as i8;
    let uc = -1 as u8;

    let ipc = pc as i32;
    let isc = sc as i32;
    let iuc = uc as u32;

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