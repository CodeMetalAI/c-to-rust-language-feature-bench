// Define structs with fields
struct S1 {
    v1: i32,
    s2p: *mut S2,
}

struct S2 {
    v2: i32,
    s1p: *mut S1,
}

fn main() {
    // Allocate memory for S1 and S2 instances
    let a = Box::new(S1 {
        v1: 10,
        s2p: Box::into_raw(Box::new(S2 {
            v2: 20,
            s1p: Box::into_raw(Box::new(S1 {
                v1: 10,
                s2p: std::ptr::null_mut(),
            })),
        })),
    });
    let b = Box::from_raw(a.s2p as *mut S2);

    // Check if field access works
    assert_eq!(b.v2, 20);
    assert_eq!((*b.s1p).v1, 10);
    assert_eq!(a.s2p as *const _ as *const S1, &*b.s1p);

    // Clean up
    Box::from_raw(a.s2p as *mut S1);
    Box::from_raw(b.s1p as *mut S1);
    Box::from_raw(a.s2p);
    Box::from_raw(b.s2p);
}