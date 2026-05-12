static mut I1: i32 = 1;
static I2: i32 = 2;
static I3: i32 = 3;

static mut I4: i32 = 0;
static mut I5: i32 = 0;

fn main() -> i32 {
    unsafe {
        if I1 != 1 {
            return 1;
        }
        if I2 != 2 {
            return 2;
        }
        if I3 != 3 {
            return 3;
        }

        if I4 != 0 {
            return 4;
        }
        if I5 != 0 {
            return 5;
        }

        let p_i1 = &I1 as *const i32;
        let p_i2 = &I2 as *const i32;
        let p_i4 = &I4 as *const i32;
        let p_i5 = &I5 as *const i32;

        if p_i1 != &I1 as *const i32 {
            return 6;
        }
        if p_i2 != &I2 as *const i32 {
            return 7;
        }
        if p_i4 != &I4 as *const i32 {
            return 8;
        }
        if p_i5 != &I5 as *const i32 {
            return 9;
        }

        I1 = 10;
        I2 = 20; // This line will not compile because I2 is immutable
        I4 = 30;
        I5 = 40;

        if *p_i1 != 10 {
            return 10;
        }
        if *p_i2 != 20 {
            return 11; // This line will not compile because I2 is immutable
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