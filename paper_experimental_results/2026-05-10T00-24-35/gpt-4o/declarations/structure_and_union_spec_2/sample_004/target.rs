use std::mem;
use std::ptr;

struct S {
    n: i32,
    d: [f64; 0], // Use a zero-length array to simulate C flexible array member
}

struct Ss {
    n: i32,
}

fn main() {
    let size_s = mem::size_of::<S>();
    let size_ss = mem::size_of::<Ss>();
    
    if size_s < size_ss {
        std::process::exit(1);
    }
    
    let offset_d = mem::size_of::<S>();
    if offset_d != size_s {
        std::process::exit(1);
    }
    
    let s1_size = size_s + mem::size_of::<f64>() * 8;
    let s2_size = size_s + mem::size_of::<f64>() * 5;
    
    let s1_ptr = unsafe { libc::malloc(s1_size) as *mut S };
    let s2_ptr = unsafe { libc::malloc(s2_size) as *mut S };
    
    if s1_ptr.is_null() || s2_ptr.is_null() {
        std::process::exit(1);
    }
    
    unsafe {
        let s1_data = s1_ptr.add(1) as *mut f64;
        let s2_data = s2_ptr.add(1) as *mut f64;
        
        *s1_data = 42.0;
        *s2_data = 24.0;
        
        if *s1_data != 42.0 || *s2_data != 24.0 {
            std::process::exit(1);
        }

        libc::free(s1_ptr as *mut libc::c_void);
        libc::free(s2_ptr as *mut libc::c_void);
    }
}