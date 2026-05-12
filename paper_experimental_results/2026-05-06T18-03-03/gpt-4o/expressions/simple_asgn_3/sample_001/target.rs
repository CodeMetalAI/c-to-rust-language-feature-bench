fn main() {
    let c = 'A';
    let p: *const char = &c;
    let cpp: *const *const char = &p;

    // Dereferencing raw pointers is unsafe, so we need to use unsafe block
    let p = unsafe { *cpp as *mut char };
    unsafe {
        *p = '\0';
    }

    if c != 'A' {
        std::process::exit(1);
    }
}