fn main() {
    let mut i1: i32 = 1;
    let i2: i32 = 2;
    let i3: i32 = 3;
    let mut i4: i32 = 0;
    let i5: i32 = 0;

    let p_i1: *const i32 = &i1;
    let p_i2: *const i32 = &i2;
    let p_i4: *const i32 = &i4;
    let p_i5: *const i32 = &i5;

    let i1: i32 = 1;
    let i2: i32 = 2;
    let i3: i32 = 3;
    let i4: i32 = 0;
    let i5: i32 = 0;

    let p_i1: *const i32 = &i1;
    let p_i2: *const i32 = &i2;
    let p_i3: *const i32 = &i3;
    let p_i4: *const i32 = &i4;
    let p_i5: *const i32 = &i5;

    let p_i1: *mut i32 = &mut i1;
    let p_i2: *mut i32 = &mut i2;
    let p_i3: *mut i32 = &mut i3;
    let p_i4: *mut i32 = &mut i4;
    let p_i5: *mut i32 = &mut i5;

    if i1 != 1 {
        return 1;
    }
    if i2 != 2 {
        return 2;
    }
    if i3 != 3 {
        return 3;
    }

    if i4 != 0 {
        return 4;
    }
    if i5 != 0 {
        return 5;
    }

    if p_i1 != &i1 {
        return 6;
    }
    if p_i2 != &i2 {
        return 7;
    }
    if p_i3 != &i3 {
        return 8;
    }
    if p_i4 != &i4 {
        return 9;
    }
    if p_i5 != &i5 {
        return 10;
    }

    *p_i1 = 10;
    *p_i2 = 20;
    *p_i3 = 30;
    *p_i4 = 40;
    *p_i5 = 40;

    if *p_i1 != 10 {
        return 11;
    }
    if *p_i2 != 20 {
        return 12;
    }
    if *p_i3 != 30 {
        return 13;
    }
    if *p_i4 != 40 {
        return 14;
    }
    if *p_i5 != 40 {
        return 15;
    }

    return 0;
}