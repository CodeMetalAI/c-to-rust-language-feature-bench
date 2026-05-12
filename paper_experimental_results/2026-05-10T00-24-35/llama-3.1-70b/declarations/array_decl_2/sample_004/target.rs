#![allow(non_snake_case)]

const OFFSETOF<T, M>(_: &T) -> usize {
    let ptr: *const T = std::ptr::null();
    (ptr as *const u8).offset(M::OFFSET) as usize
}

trait Offset {
    const OFFSET: isize;
}

impl Offset for HoldA {
    const OFFSET: isize = std::mem::size_of::<i32>();
}

struct HoldP {
    p: *const i32,
}

struct HoldA {
    tag: i32,
    a: [i32],
}

static mut y: [i32; 7] = [0; 7];

static mut backing: [i32; 3] = [10, 20, 30];
static mut x: *const i32 = &backing[0];

fn sum_ptr(p: *const i32, n: usize) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += unsafe { *p.offset(i as isize) };
        i += 1;
    }
    s
}

fn sum_arr7(a: &[i32]) -> i32 {
    a.iter().sum()
}

fn mutate_through_pointer(p: &mut i32) -> i32 {
    unsafe { *p.offset(1) += 5 };
    p + unsafe { *p.offset(1) } + unsafe { *p.offset(2) }
}

fn main() {
    unsafe {
        y[0] = 1;
        y[1] = 2;
        y[2] = 3;
        y[3] = 4;
        y[4] = 5;
        y[5] = 6;
        y[6] = 7;
    }

    if sum_arr7(unsafe { &y })!= (1 + 2 + 3 + 4 + 5 + 6 + 7) {
        std::process::exit(1);
    }

    if sum_ptr(x, 3)!= (10 + 20 + 30) {
        std::process::exit(2);
    }

    let mut ptr = unsafe { x.as_mut().unwrap() };
    if mutate_through_pointer(ptr)!= (10 + 25 + 30) {
        std::process::exit(3);
    }

    unsafe {
        if backing[1]!= 25 {
            std::process::exit(4);
        }
    }

    {
        let t = unsafe { &y };
        if t[6]!= 7 {
            std::process::exit(5);
        }
    }

    {
        let hp = HoldP { p: unsafe { &y } };
        if unsafe { *hp.p }!= 1 {
            std::process::exit(6);
        }

        let ha = unsafe { &*(hp.p as *const _ as *const HoldA) };
        if (ha as *const _ as *const u8).offset(HoldA::OFFSET as isize)!= ha.a.as_ptr() as *const u8 {
            std::process::exit(7);
        }

        if ha.a[2]!= 3 {
            std::process::exit(8);
        }
    }

    unsafe {
        if y[0]!= 1 {
            std::process::exit(9);
        }
    }

    std::process::exit(0);
}