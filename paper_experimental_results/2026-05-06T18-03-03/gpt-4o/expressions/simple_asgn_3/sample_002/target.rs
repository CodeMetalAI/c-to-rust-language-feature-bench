fn main() {
    let c = 'A';
    let p: &mut u8 = &mut 0;
    let cpp: &mut &u8 = &mut p;

    *cpp = &c as *const char as *const u8;

    if c != 'A' {
        std::process::exit(1);
    }
}