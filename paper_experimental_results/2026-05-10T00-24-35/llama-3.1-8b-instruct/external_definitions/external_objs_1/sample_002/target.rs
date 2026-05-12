fn main() {
    let i1 = 1;
    static mut i2: i32 = 2;
    let i3 = 3;

    let i4 = 0;
    static mut i5: i32 = 0;

    let i1 = 1;
    static mut i2: i32 = 2;
    let i3 = 3;
    let i4 = 0;
    static mut i5: i32 = 0;

    let i1_ref = &mut i1;
    let i2_ref = &mut i2;
    let i4_ref = &mut i4;
    let i5_ref = &mut i5;

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

    if i1_ref != &i1 {
        return 6;
    }
    if i2_ref != &i2 {
        return 7;
    }
    if i4_ref != &i4 {
        return 8;
    }
    if i5_ref != &i5 {
        return 9;
    }

    *i1_ref = 10;
    *i2_ref = 20;
    *i4_ref = 30;
    *i5_ref = 40;

    if *i1_ref != 10 {
        return 10;
    }
    if *i2_ref != 20 {
        return 11;
    }
    if *i4_ref != 30 {
        return 12;
    }
    if *i5_ref != 40 {
        return 13;
    }

    println!("All tests passed");
}