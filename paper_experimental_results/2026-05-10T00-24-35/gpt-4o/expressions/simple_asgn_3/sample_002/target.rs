fn main() {
    let c: u8 = b'A';

    let p: &mut u8 = &mut 0;
    let cpp: *const &u8 = &p;

    unsafe {
        **(cpp as *const *const u8 as *mut *mut u8) = c;
        *p = 0;
    }

    if c != b'A' {
        std::process::exit(1);
    }
}