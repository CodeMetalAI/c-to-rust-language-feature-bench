// array_decl_2.rs
use std::mem;

const BACKING: [i32; 3] = [10, 20, 30];

fn sum_ptr(p: &[i32], n: usize) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += p[i];
        i += 1;
    }
    s
}

fn sum_arr7(a: [i32; 7]) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < 7 {
        s += a[i];
        i += 1;
    }
    s
}

fn mutate_through_pointer(p: &mut [i32]) -> i32 {
    p[1] += 5;
    p[0] + p[1] + p[2]
}

fn main() {
    let mut y = [1, 2, 3, 4, 5, 6, 7];

    assert_eq!(sum_arr7(y), 1 + 2 + 3 + 4 + 5 + 6 + 7);

    let x = &BACKING;
    assert_eq!(sum_ptr(x, 3), 10 + 20 + 30);

    let mut x = &mut BACKING;
    assert_eq!(mutate_through_pointer(x), 10 + 25 + 30);

    assert_eq!(BACKING[1], 25);

    {
        let t = &y;
        assert_eq!(t[6], 7);
    }

    {
        let hp = HoldP { p: &y };
        assert_eq!(hp.p[0], 1);

        let ha = unsafe { std::mem::transmute::<&[i32], &HoldA>(y) };
        assert_eq!((ha as *const _ as usize) + mem::size_of::<HoldA>() as usize,
                  (ha as *const _ as usize) + mem::size_of::<i32>() as usize);

        assert_eq!(ha.a[2], 3);
    }

    assert_eq!(y[0], 1);
}

struct HoldP {
    p: *const i32,
}

struct HoldA {
    tag: i32,
    a: [i32; 3],
}