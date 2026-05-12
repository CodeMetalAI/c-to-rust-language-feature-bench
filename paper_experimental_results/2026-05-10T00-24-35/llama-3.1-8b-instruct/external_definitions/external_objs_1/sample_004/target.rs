fn main() {
    let mut i1: i32 = 1;
    let mut i2: i32 = 2;
    let mut i3: i32 = 3;
    let mut i4: i32 = 0;
    let mut i5: i32 = 0;

    if i1!= 1 {
        return 1;
    }
    if i2!= 2 {
        return 2;
    }
    if i3!= 3 {
        return 3;
    }

    if i4!= 0 {
        return 4;
    }
    if i5!= 0 {
        return 5;
    }

    let p_i1 = &mut i1;
    let p_i2 = &mut i2;
    let p_i4 = &mut i4;
    let p_i5 = &mut i5;

    if p_i1!= &i1 {
        return 6;
    }
    if p_i2!= &i2 {
        return 7;
    }
    if p_i4!= &i4 {
        return 8;
    }
    if p_i5!= &i5 {
        return 9;
    }

    *p_i1 = 10;
    *p_i2 = 20;
    *p_i4 = 30;
    *p_i5 = 40;

    if *p_i1!= 10 {
        return 10;
    }
    if *p_i2!= 20 {
        return 11;
    }
    if *p_i4!= 30 {
        return 12;
    }
    if *p_i5!= 40 {
        return 13;
    }

    return 0;
}