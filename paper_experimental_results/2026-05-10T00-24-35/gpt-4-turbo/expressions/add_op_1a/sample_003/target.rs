fn main() {
    let mut a = [[0; 3]; 4];

    {
        let mut p = &mut a[1..];
        p[0][2] = 99;
    }

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    let n = {
        let p = &a[1] as *const _;
        let base = &a[0] as *const _;
        unsafe { p.offset_from(base) as isize }
    };
    if n != 1 {
        std::process::exit(2);
    }
}