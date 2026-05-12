#[derive(Debug)]
struct S1 {
    v1: i32,
    s2p: *mut S2,
}

#[derive(Debug)]
struct S2 {
    v2: i32,
    s1p: *mut S1,
}

fn main() {
    let mut a = S1 {
        v1: 10,
        s2p: std::ptr::null_mut(),
    };
    let mut b = S2 {
        v2: 20,
        s1p: std::ptr::null_mut(),
    };

    let a_ptr = &mut a as *mut S1;
    let b_ptr = &mut b as *mut S2;

    unsafe {
        a.s2p = b_ptr;
        b.s1p = a_ptr;

        if (*a.s2p).v2 != 20 {
            return 1;
        }

        if (*b.s1p).v1 != 10 {
            return 2;
        }

        if (*(*a.s2p).s1p).v1 != 10 {
            return 3;
        }
    }

    0
}