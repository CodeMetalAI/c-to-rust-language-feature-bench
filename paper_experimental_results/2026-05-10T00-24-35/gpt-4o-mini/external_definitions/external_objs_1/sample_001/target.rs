static mut I1: i32 = 1;
static mut I2: i32 = 2;
static mut I3: i32 = 3;

static mut I4: i32 = 0;
static mut I5: i32 = 0;

fn main() -> i32 {
    unsafe {
        let p_i1: *const i32 = &I1;
        let p_i2: *const i32 = &I2;
        let p_i4: *const i32 = &I4;
        let p_i5: *const i32 = &I5;

        if I1 != 1 { return 1; }
        if I2 != 2 { return 2; }
        if I3 != 3 { return 3; }
        if I4 != 0 { return 4; }
        if I5 != 0 { return 5; }

        if p_i1 != &I1 as *const _ { return 6; }
        if p_i2 != &I2 as *const _ { return 7; }
        if p_i4 != &I4 as *const _ { return 8; }
        if p_i5 != &I5 as *const _ { return 9; }

        I1 = 10;
        I2 = 20;
        I4 = 30;
        I5 = 40;

        if *p_i1 != 10 { return 10; }
        if *p_i2 != 20 { return 11; }
        if *p_i4 != 30 { return 12; }
        if *p_i5 != 40 { return 13; }
    }
    0
}