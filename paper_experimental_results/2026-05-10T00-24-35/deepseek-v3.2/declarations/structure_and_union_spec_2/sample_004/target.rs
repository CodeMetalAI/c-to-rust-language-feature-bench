use std::alloc::{alloc, dealloc, Layout};
use std::mem::{size_of, align_of};

fn main() {
    #[repr(C)]
    struct S {
        n: i32,
        d: [f64; 0],
    }

    #[repr(C)]
    struct Ss {
        n: i32,
    }

    if size_of::<S>() < size_of::<Ss>() {
        std::process::exit(1);
    }

    let d_offset = {
        let dummy = S { n: 0, d: [] };
        let base = &dummy as *const _ as *const u8;
        let field = &dummy.d as *const _ as *const u8;
        (field as usize) - (base as usize)
    };

    if d_offset != size_of::<S>() {
        std::process::exit(1);
    }

    let layout1 = Layout::from_size_align(
        size_of::<S>() + size_of::<f64>() * 8,
        align_of::<S>(),
    ).unwrap();

    let layout2 = Layout::from_size_align(
        size_of::<S>() + size_of::<f64>() * 5,
        align_of::<S>(),
    ).unwrap();

    let s1_ptr = unsafe { alloc(layout1) as *mut S };
    let s2_ptr = unsafe { alloc(layout2) as *mut S };

    if s1_ptr.is_null() || s2_ptr.is_null() {
        std::process::exit(1);
    }

    unsafe {
        (*s1_ptr).n = 0;
        (*s2_ptr).n = 0;
    }

    unsafe {
        let d1_slice = std::slice::from_raw_parts_mut(
            (*s1_ptr).d.as_mut_ptr() as *mut f64,
            8,
        );
        d1_slice[0] = 42.0;

        let d2_slice = std::slice::from_raw_parts_mut(
            (*s2_ptr).d.as_mut_ptr() as *mut f64,
            5,
        );
        d2_slice[0] = 24.0;
    }

    unsafe {
        let d1_slice = std::slice::from_raw_parts(
            (*s1_ptr).d.as_ptr() as *const f64,
            8,
        );
        let d2_slice = std::slice::from_raw_parts(
            (*s2_ptr).d.as_ptr() as *const f64,
            5,
        );

        if d1_slice[0] != 42.0 || d2_slice[0] != 24.0 {
            dealloc(s1_ptr as *mut u8, layout1);
            dealloc(s2_ptr as *mut u8, layout2);
            std::process::exit(1);
        }
    }

    unsafe {
        dealloc(s1_ptr as *mut u8, layout1);
        dealloc(s2_ptr as *mut u8, layout2);
    }

    std::process::exit(0);
}