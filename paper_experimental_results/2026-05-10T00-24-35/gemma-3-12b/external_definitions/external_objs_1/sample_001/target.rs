fn main() {
    static mut i1: i32 = 1;
    static mut i2: i32 = 2;
    static mut i3: i32 = 3;
    static mut i4: i32 = 0;
    static mut i5: i32 = 0;

    unsafe {
        if *i1 != 1 {
            return 1;
        }
        if *i2 != 2 {
            return 2;
        }
        if *i3 != 3 {
            return 3;
        }

        if *i4 != 0 {
            return 4;
        }
        if *i5 != 0 {
            return 5;
        }
    }

    unsafe {
        let p_i1 = &mut *i1;
        let p_i2 = &mut *i2;
        let p_i4 = &mut *i4;
        let p_i5 = &mut *i5;

        if p_i1 as *mut i32 != &mut *i1 as *mut i32 {
            return 6;
        }
        if p_i2 as *mut i32 != &mut *i2 as *mut i32 {
            return 7;
        }
        if p_i4 as *mut i32 != &mut *i4 as *mut i32 {
            return 8;
        }
        if p_i5 as *mut i32 != &mut *i5 as *mut i32 {
            return 9;
        }
    }

    unsafe {
        *i1 = 10;
        *i2 = 20;
        *i4 = 30;
        *i5 = 40;

        if *p_i1 != 10 {
            return 10;
        }
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

    0
}