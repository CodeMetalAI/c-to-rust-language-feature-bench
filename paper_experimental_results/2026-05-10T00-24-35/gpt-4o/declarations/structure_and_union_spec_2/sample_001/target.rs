fn main() {
    use std::mem;
    use std::ptr;

    struct S {
        n: i32,
        d: [f64; 0],
    }

    struct Ss {
        n: i32,
    }

    if mem::size_of::<S>() < mem::size_of::<Ss>() {
        std::process::exit(1);
    }

    if mem::align_of::<S>() != mem::size_of::<S>() {
        std::process::exit(1);
    }

    let s1_size = mem::size_of::<S>() + mem::size_of::<f64>() * 8;
    let s2_size = mem::size_of::<S>() + mem::size_of::<f64>() * 5;

    let s1_raw = unsafe { libc::malloc(s1_size) as *mut S };
    let s2_raw = unsafe { libc::malloc(s2_size) as *mut S };

    if s1_raw.is_null() || s2_raw.is_null() {
        unsafe {
            libc::free(s1_raw as *mut _);
            libc::free(s2_raw as *mut _);
        }
        std::process::exit(1);
    }

    unsafe {
        ptr::write(s1_raw.cast::<f64>().add(1), 42.0);
        ptr::write(s2_raw.cast::<f64>().add(1), 24.0);

        if *s1_raw.cast::<f64>().add(1) != 42.0 || *s2_raw.cast::<f64>().add(1) != 24.0 {
            libc::free(s1_raw as *mut _);
            libc::free(s2_raw as *mut _);
            std::process::exit(1);
        }

        libc::free(s1_raw as *mut _);
        libc::free(s2_raw as *mut _);
    }
}