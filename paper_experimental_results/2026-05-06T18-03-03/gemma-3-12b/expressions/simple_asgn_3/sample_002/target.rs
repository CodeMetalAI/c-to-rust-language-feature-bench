fn main() {
    let mut cpp: &'static [const char] = [&mut 0];
    let mut p: char = 0;
    let c = 'A';

    let cpp_ptr: *mut &'static [const char] = &mut cpp as *mut &'static [const char];
    let p_ptr: *mut char = &mut p as *mut char;

    unsafe {
        *cpp_ptr = &[c];
        *p_ptr = 0;
    }

    if c != 'A' {
        return 1;
    }

    0
}