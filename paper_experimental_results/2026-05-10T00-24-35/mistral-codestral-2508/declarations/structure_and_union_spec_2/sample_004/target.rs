use std::mem;

fn main() {
    #[repr(C)]
    struct S {
        n: i32,
        d: [f64; 0],
    }

    #[repr(C)]
    struct SS {
        n: i32,
    }

    if mem::size_of::<S>() < mem::size_of::<SS>() {
        std::process::exit(1);
    }

    let offset = unsafe { &(*(0 as *const S)).d as *const _ as usize };
    if offset != mem::size_of::<S>() {
        std::process::exit(1);
    }

    let s1 = unsafe {
        let layout = std::alloc::Layout::array::<f64>(8).unwrap();
        let ptr = std::alloc::alloc(layout) as *mut S;
        if ptr.is_null() {
            std::process::exit(1);
        }
        &mut *ptr
    };

    let s2 = unsafe {
        let layout = std::alloc::Layout::array::<f64>(5).unwrap();
        let ptr = std::alloc::alloc(layout) as *mut S;
        if ptr.is_null() {
            std::process::exit(1);
        }
        &mut *ptr
    };

    unsafe {
        s1.d[0] = 42.0;
        s2.d[0] = 24.0;

        if s1.d[0] != 42.0 || s2.d[0] != 24.0 {
            std::process::exit(1);
        }
    }

    unsafe {
        let layout = std::alloc::Layout::array::<f64>(8).unwrap();
        std::alloc::dealloc(s1 as *mut S as *mut u8, layout);

        let layout = std::alloc::Layout::array::<f64>(5).unwrap();
        std::alloc::dealloc(s2 as *mut S as *mut u8, layout);
    }
}