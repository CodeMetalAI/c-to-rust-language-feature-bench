use std::cell::RefCell;
use std::process::exit;

thread_local! {
    static ARR: RefCell<[i32; 256]> = RefCell::new([0; 256]);
}

fn f(_a: u8, b: u8) -> i32 {
    ARR.with(|arr| {
        let mut arr = arr.borrow_mut();
        arr[b as usize] = 1;
    });
    b as i32
}

fn g(a: i32, b: i32) -> i32 {
    ARR.with(|arr| {
        let mut arr = arr.borrow_mut();
        let idx = b as u8 as usize;
        arr[idx] += b;
    });
    a + b
}

fn main() {
    let a: i8 = 0;
    let b: i8 = -1;

    let r1 = f(a as u8, b as u8);
    let r2 = g(a as i32, b as i32);

    if r1 != 255 {
        exit(1);
    }
    if r2 != -1 {
        exit(2);
    }

    let ub = b as u8;
    let val = ARR.with(|arr| arr.borrow()[ub as usize]);
    if val != 0 {
        exit(3);
    }
    exit(0);
}