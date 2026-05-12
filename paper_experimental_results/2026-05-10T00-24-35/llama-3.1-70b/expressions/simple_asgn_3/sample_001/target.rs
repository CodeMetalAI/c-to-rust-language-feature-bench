fn main() {
    let c: char = 'A';
    let p: *mut char = &mut c as *mut char;
    let cpp: *mut *const char = &mut p as *mut *const char;
    unsafe {
        *cpp = &c;
        *p = 0;
    }
    if c!= 'A' {
        std::process::exit(1);
    }
}