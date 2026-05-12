fn main() {
    let mut i1 = 1;
    static mut i2: i32 = 2;
    static mut i3: i32 = 3;
    static mut i4: i32 = 0;
    static mut i5: i32 = 0;

    unsafe {
        if i1 != 1 {
            return 1;
        }
        if *i2 != 2 {
            return 2;
        }
        if *i3 != 3 {
            return 3;
        }

        if i4 != 0 {
            return 4;
        }
        if i5 != 0 {
            return 5;
        }

        if &i1 as *const i32 != &i1 {
            return 6;
        }
        if &i2 as *const i32 != &i2 {
            return 7;
        }
        if &i4 as *const i32 != &i4 {
            return 8;
        }
        if &i5 as *const i32 != &i5 {
            return 9;
        }

        i1 = 10;
        *i2 = 20;
        i4 = 30;
        i5 = 40;

        if *&i1 as *const i32 != 10 {
            return 10;
        }
        if *&i2 as *const i32 != 20 {
            return 11;
        }
        if *&i4 as *const i32 != 30 {
            return 12;
        }
        if *&i5 as *const i32 != 40 {
            return 13;
        }
    }

    0
}