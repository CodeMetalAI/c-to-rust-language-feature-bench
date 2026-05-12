fn main() {
    let mut cpp: &'static [const char] = [&mut 0 as &'static [const char]];
    let mut p: char = 'A';

    let ptr_p: *mut char = &mut p;
    let ptr_cpp: *const &'static [const char] = &cpp as *const &'static [const char];

    unsafe {
        cpp[0] = ptr_p as *const char;
    }

    let mut mutable_p: char = 'A';
    unsafe {
        p = mutable_p;
    }

    if p != 'A' {
        return 1;
    }

    0
}