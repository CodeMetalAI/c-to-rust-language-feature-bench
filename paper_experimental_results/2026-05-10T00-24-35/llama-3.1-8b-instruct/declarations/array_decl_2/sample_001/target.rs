// array_decl_2.rs
const SIZE: usize = 7;
const SIZE_BACKING: usize = 3;

static mut BACKING: [i32; SIZE_BACKING] = [10, 20, 30];

#[repr(C)]
struct HoldP {
    p: *const i32,
}

#[repr(C)]
struct HoldA {
    tag: i32,
    a: [i32; 3],
}

fn sum_ptr(p: *const i32, n: usize) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += *p.offset(i as isize);
        i += 1;
    }
    s
}

fn sum_arr7(a: &[i32]) -> i32 {
    a.iter().sum()
}

fn mutate_through_pointer(p: *mut i32) -> i32 {
    unsafe {
        *p.offset(1) += 5;
        *p.offset(0) + *p.offset(1) + *p.offset(2)
    }
}

fn main() {
    let mut y = [1, 2, 3, 4, 5, 6, 7];
    assert_eq!(sum_arr7(&y), 1 + 2 + 3 + 4 + 5 + 6 + 7);

    let x = unsafe { &mut BACKING };
    assert_eq!(sum_ptr(x.as_ptr(), SIZE_BACKING), 10 + 20 + 30);

    assert_eq!(mutate_through_pointer(x.as_mut_ptr()), 10 + 25 + 30);

    assert_eq!(*unsafe { BACKING.get_unchecked_mut(1) }, 25);

    {
        let t = &y;
        assert_eq!(t[6], 7);
    }

    {
        let hp = HoldP { p: y.as_ptr() };
        assert_eq!(*hp.p, 1);

        let ha = unsafe { &*(y.as_ptr() as *const HoldA) };
        assert_eq!(ha.a[0], 1);
        assert_eq!(ha.a[1], 2);
        assert_eq!(ha.a[2], 3);
    }

    assert_eq!(y[0], 1);
}