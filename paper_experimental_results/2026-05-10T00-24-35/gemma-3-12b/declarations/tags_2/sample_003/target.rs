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

    a.s2p = &mut b as *mut S2;
    b.s1p = &mut a as *mut S1;

    if unsafe { (*a.s2p).v2 } != 20 {
        return std::process::exit(1);
    }

    if unsafe { (*b.s1p).v1 } != 10 {
        return std::process::exit(2);
    }

    if unsafe { (*a.s2p).s1p } != &a {
        return std::process::exit(3);
    }

    std::process::exit(0);
}