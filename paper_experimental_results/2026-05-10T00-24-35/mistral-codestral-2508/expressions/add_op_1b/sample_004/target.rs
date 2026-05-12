fn main() {
    let mut a: [[i32; 3]; 4] = [[0; 3]; 4];

    let mut p: &mut [i32; 3] = &mut a[0];

    p = &mut a[1];
    p[2] = 99;

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    let diff = p.as_ptr().offset_from(a.as_ptr()) as isize;
    if diff != 1 {
        std::process::exit(2);
    }
}