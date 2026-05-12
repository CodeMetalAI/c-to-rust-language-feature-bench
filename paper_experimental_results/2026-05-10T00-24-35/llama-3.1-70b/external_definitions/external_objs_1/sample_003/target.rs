fn main() {
    let i1 = 1;
    static mut i2: i32 = 2;
    static mut i3: i32 = 3;
    let mut i4 = 0;
    static mut i5: i32 = 0;

    let p_i1 = &i1 as *const i32;
    static mut p_i2: *const i32 = &i2;
    static mut p_i4: *mut i32 = &mut i4;
    static mut p_i5: *mut i32 = &mut i5;

    if i1 != 1 {
        return 1;
    }
    unsafe {
        if *i2 != 2 {
            return 2;
        }
        if *i3 != 3 {
            return 3;
        }
    }
    if i4 != 0 {
        return 4;
    }
    unsafe {
        if *i5 != 0 {
            return 5;
        }
    }

    if p_i1 != &i1 as *const i32 {
        return 6;
    }
    unsafe {
        if *p_i2 != &i2 {
            return 7;
        }
        if *p_i4 != &i4 {
            return 8;
        }
        if *p_i5 != &i5 {
            return 9;
        }
    }

    i1 = 10;
    unsafe {
        *i2 = 20;
        *i4 = 30;
        *i5 = 40;
    }

    if unsafe { *p_i1 } != 10 {
        return 10;
    }
    unsafe {
        if *p_i2 != 20 {
            return 11;
        }
        if *p_i4 != 30 {
            return 12;
        }
        if *p_i5 != 40 {
            return 13;
        }
    }

    std::process::exit(0);
}