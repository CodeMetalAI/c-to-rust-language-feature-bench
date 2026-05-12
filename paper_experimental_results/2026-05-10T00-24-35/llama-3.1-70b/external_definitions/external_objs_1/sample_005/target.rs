fn main() {
    static mut i1: i32 = 1;
    static mut i2: i32 = 2;
    static mut i3: i32 = 3;
    static mut i4: i32 = 0;
    static mut i5: i32 = 0;

    static mut p_i1: *mut i32 = &mut i1;
    static mut p_i2: *mut i32 = &mut i2;
    static mut p_i4: *mut i32 = &mut i4;
    static mut p_i5: *mut i32 = &mut i5;

    if unsafe { *p_i1 }!= 1 {
        std::process::exit(1);
    }
    if unsafe { *p_i2 }!= 2 {
        std::process::exit(2);
    }
    if unsafe { i3 }!= 3 {
        std::process::exit(3);
    }

    if unsafe { *p_i4 }!= 0 {
        std::process::exit(4);
    }
    if unsafe { *p_i5 }!= 0 {
        std::process::exit(5);
    }

    if unsafe { p_i1 }!= &mut i1 {
        std::process::exit(6);
    }
    if unsafe { p_i2 }!= &mut i2 {
        std::process::exit(7);
    }
    if unsafe { p_i4 }!= &mut i4 {
        std::process::exit(8);
    }
    if unsafe { p_i5 }!= &mut i5 {
        std::process::exit(9);
    }

    unsafe {
        *p_i1 = 10;
        *p_i2 = 20;
        *p_i4 = 30;
        *p_i5 = 40;
    }

    if unsafe { *p_i1 }!= 10 {
        std::process::exit(10);
    }
    if unsafe { *p_i2 }!= 20 {
        std::process::exit(11);
    }
    if unsafe { *p_i4 }!= 30 {
        std::process::exit(12);
    }
    if unsafe { *p_i5 }!= 40 {
        std::process::exit(13);
    }

    std::process::exit(0);
}