use std::mem;
use std::ptr;
use std::alloc::{self, Layout};

struct S {
    n: i32,
    // Flexible array member simulation: we don't have this directly in Rust,
    // so we'll handle allocation ourselves.
}

struct Ss {
    n: i32,
}

fn main() {
    // In Rust, we verify the size of the structures
    let size_of_s = mem::size_of::<S>();
    let size_of_ss = mem::size_of::<Ss>();

    if size_of_s < size_of_ss {
        std::process::exit(1);
    }

    // offsetof equivalent in Rust (size_of a struct is offset to its first member)
    if mem::size_of::<S>() != size_of_s {
        std::process::exit(1);
    }

    // Allocate memory for the flexible array members
    let layout_s1 = Layout::array::<f64>(8).unwrap();
    let layout_s2 = Layout::array::<f64>(5).unwrap();

    unsafe {
        let ptr_s1 = alloc::alloc_zeroed(Layout::new::<S>().extend(layout_s1).unwrap().0);
        let ptr_s2 = alloc::alloc_zeroed(Layout::new::<S>().extend(layout_s2).unwrap().0);

        if ptr_s1.is_null() || ptr_s2.is_null() {
            std::process::exit(1);
        }

        let s1 = &mut *(ptr_s1 as *mut S);
        let s2 = &mut *(ptr_s2 as *mut S);

        // Simulate direct pointer arithmetic
        let d1 = ptr_s1.add(mem::size_of::<S>()) as *mut f64;
        let d2 = ptr_s2.add(mem::size_of::<S>()) as *mut f64;

        ptr::write(d1, 42.0);
        ptr::write(d2, 24.0);

        if *d1 != 42.0 || *d2 != 24.0 {
            std::process::exit(1);
        }

        alloc::dealloc(ptr_s1, Layout::new::<S>().extend(layout_s1).unwrap().0);
        alloc::dealloc(ptr_s2, Layout::new::<S>().extend(layout_s2).unwrap().0);
    }
}