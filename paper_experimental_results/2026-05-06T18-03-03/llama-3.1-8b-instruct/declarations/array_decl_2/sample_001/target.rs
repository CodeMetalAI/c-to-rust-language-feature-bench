// array_decl_2.rs
use std::ptr;

const BACKING: [usize; 3] = [10, 20, 30];

struct HoldP {
    p: *const usize,
}

struct HoldA {
    tag: usize,
    a: [usize; 6],
}

fn sum_ptr(p: *const usize, n: usize) -> usize {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += *p.offset(i as isize);
        i += 1;
    }
    s
}

fn sum_arr7(a: &[usize]) -> usize {
    let mut s = 0;
    let mut i = 0;
    while i < a.len() {
        s += a[i];
        i += 1;
    }
    s
}

fn mutate_through_pointer(p: *mut usize) -> usize {
    unsafe {
        *p.offset(1) += 5;
        *p.offset(0) + *p.offset(1) + *p.offset(2)
    }
}

fn main() {
    let mut y = [1, 2, 3, 4, 5, 6, 7];

    assert_eq!(sum_arr7(&y), 1 + 2 + 3 + 4 + 5 + 6 + 7);

    let x = BACKING.as_ptr();
    assert_eq!(sum_ptr(x, 3), 10 + 20 + 30);

    assert_eq!(mutate_through_pointer(x), 10 + 25 + 30);

    assert_eq!(*x.offset(1), 25);

    {
        let t = &y;
        assert_eq!(*t.offset(6), 7);
    }

    {
        let hp = HoldP { p: y.as_ptr() };
        assert_eq!(*hp.p, 1);

        let ha = unsafe { &*(hp.p as *const HoldA) };
        assert_eq!(ha as *const _ as usize, hp.p as usize + std::mem::size_of::<usize>() as usize);
        assert_eq!(*ha.a.offset(2), 3);
    }

    assert_eq!(*y.as_ptr(), 1);
}