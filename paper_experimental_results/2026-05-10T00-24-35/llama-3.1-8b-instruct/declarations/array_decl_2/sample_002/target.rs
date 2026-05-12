fn main() {
    let y = [1, 2, 3, 4, 5, 6, 7];
    let x = & backups[0];

    assert_eq!(sum_arr7(&y), 28);
    assert_eq!(sum_ptr(x, 3), 60);
    assert_eq!(mutate_through_pointer(x), 65);
    assert_eq!(backups[1], 25);
    {
        let t = &y;
        assert_eq!(t[6], 7);
    }
    {
        struct HoldP {
            p: *const i32,
        }
        let hp = HoldP { p: &y };
        assert_eq!(*hp.p, 1);

        let ha = unsafe { &*(y as *const i32 as *const HoldA) };
        assert_eq!(unsafe { (&ha.a as *const i32).offset_of::<i32>() as usize }, std::mem::size_of::<HoldA>() as usize);
        assert_eq!(ha.a[2], 3);
    }
    assert_eq!(y[0], 1);
    0
}

struct HoldA {
    tag: i32,
    a: [i32; 3]
}

const backups: [i32; 3] = [10, 20, 30];
fn sum_ptr(_p: *const i32, _n: usize) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < _n {
        s += *_p.offset(i as isize);
        i += 1;
    }
    s
}

fn sum_arr7(_a: &[i32]) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < _a.len() {
        s += _a[i];
        i += 1;
    }
    s
}

fn mutate_through_pointer(p: *mut i32) -> i32 {
    unsafe { *p.offset(1) += 5 };
    unsafe { *p.offset(0) + *p.offset(1) + *p.offset(2) }
}

trait OffsetOf {
    fn offset_of<T>() -> usize;
}

impl<T> OffsetOf for *const T {
    fn offset_of<T: Copy>() -> usize {
        std::mem::size_of::<T>()
    }
}