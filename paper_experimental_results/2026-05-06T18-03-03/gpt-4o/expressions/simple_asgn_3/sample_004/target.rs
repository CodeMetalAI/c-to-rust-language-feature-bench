fn main() {
    let c = 'A';
    let p: &mut u8 = &mut 0;

    let cpp: &*const u8 = &(&c as *const char as *const u8);
    unsafe {
        *p = **cpp;
    }

    if c != 'A' {
        std::process::exit(1);
    }
}