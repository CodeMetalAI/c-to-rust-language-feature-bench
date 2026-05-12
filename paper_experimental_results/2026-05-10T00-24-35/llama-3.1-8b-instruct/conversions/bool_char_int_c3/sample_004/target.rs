fn main() {
    let mut pc = -1i8; // equivalent to char -1
    let mut sc = -1i8; // equivalent to signed char -1
    let mut uc = -1u8; // equivalent to unsigned char -1

    let ipc = pc as i32;
    let isc = sc as i32;
    let iuc = uc as u32;

    if pc + 0 != 1 {
        return 1;
    }
    if sc + 0 != 1 {
        return 2;
    }
    if uc + 0 != 1 {
        return 3;
    }

    if isc != -1 {
        return 4;
    }

    if -1::<i8> < 0 {
        if ipc != -1 {
            return 5;
        }
        if u32::try_from(iuc).unwrap() != 255 {
            return 6;
        }
    } else {
        if i32::try_from(ipc).unwrap() != 255 {
            return 7;
        }
        if u32::try_from(iuc).unwrap() != 255 {
            return 8;
        }
    }

    return 0;
}