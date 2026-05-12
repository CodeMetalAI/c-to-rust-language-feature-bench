use std::process;
use std::ptr;

fn main() {
    let mut i1: i32 = 1;
    let mut i2: i32 = 2;
    let mut i3: i32 = 3;
    let mut i4: i32 = 0;
    let mut i5: i32 = 0;

    let p_i1: &i32 = &i1;
    let p_i2: &i32 = &i2;
    let p_i4: &i32 = &i4;
    let p_i5: &i32 = &i5;

    if i1 != 1 {
        process::exit(1);
    }
    if i2 != 2 {
        process::exit(2);
    }
    if i3 != 3 {
        process::exit(3);
    }
    if i4 != 0 {
        process::exit(4);
    }
    if i5 != 0 {
        process::exit(5);
    }

    if !ptr::eq(p_i1, &i1) {
        process::exit(6);
    }
    if !ptr::eq(p_i2, &i2) {
        process::exit(7);
    }
    if !ptr::eq(p_i4, &i4) {
        process::exit(8);
    }
    if !ptr::eq(p_i5, &i5) {
        process::exit(9);
    }

    i1 = 10;
    i2 = 20;
    i4 = 30;
    i5 = 40;

    if *p_i1 != 10 {
        process::exit(10);
    }
    if *p_i2 != 20 {
        process::exit(11);
    }
    if *p_i4 != 30 {
        process::exit(12);
    }
    if *p_i5 != 40 {
        process::exit(13);
    }

    process::exit(0);
}