use std::alloc::{alloc, dealloc, Layout};
use std::mem::{size_of, align_of};

struct S {
    n: i32,
    d: [f64; 0],
}

struct SS {
    n: i32,
}

fn main() -> i32 {
    if size_of::<S>() < size_of::<SS>() {
        return 1;
    }

    if std::mem::offset_of!(S, d) != size_of::<S>() {
        return 1;
    }

    let layout1 = Layout::from_size_align(size_of::<S>() + size_of::<f64>() * 8, align_of::<f64>()).unwrap();
    let layout2 = Layout::from_size_align(size_of::<S>() + size_of::<f64>() * 5, align_of::<f64>()).unwrap();

    let s1 = unsafe { alloc(layout1) as *mut S };
    let s2 = unsafe { alloc(layout2) as *mut S };

    if s1.is_null() || s2.is_null() {
        unsafe {
            if !s1.is_null() {
                dealloc(s1 as *mut u8, layout1);
            }
            if !s2.is_null() {
                dealloc(s2 as *mut u8, layout2);
            }
        }
        return 1;
    }

    unsafe {
        (*s1).d[0] = 42.0;
        (*s2).d[0] = 24.0;

        if (*s1).d[0] != 42.0 || (*s2).d[0] != 24.0 {
            dealloc(s1 as *mut u8, layout1);
            dealloc(s2 as *mut u8, layout2);
            return 1;
        }

        dealloc(s1 as *mut u8, layout1);
        dealloc(s2 as *mut u8, layout2);
    }

    return 0;
}