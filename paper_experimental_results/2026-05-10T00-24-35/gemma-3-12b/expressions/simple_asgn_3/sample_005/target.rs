fn main() {
    let mut cpp: &'static [&'static str] = [&""];
    let mut p: char = 0;
    let c = 'A';

    let cpp_ptr: *const &'static str = cpp.as_ptr();
    let p_ptr: *mut char = &mut p;

    unsafe {
        *cpp_ptr = &c as &'static str;
        *p_ptr = 0;
    }

    if c != 'A' {
        return 1;
    }

    0
}