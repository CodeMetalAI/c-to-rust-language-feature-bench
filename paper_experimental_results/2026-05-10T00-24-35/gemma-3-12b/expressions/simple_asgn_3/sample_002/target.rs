fn main() {
    let mut cpp: &'static str = "";
    let mut p: String = String::new();
    let c = 'A';

    let ptr_p: *mut String = &mut p as *mut String;
    let cpp_ptr: *const &'static str = &cpp as *const &'static str;
    unsafe {
        *cpp_ptr = &c as *const char as &'static str;
        (*ptr_p).push(0);
    }

    if c != 'A' {
        return 1;
    }

    0
}