fn main() {
    let mut a = [[0; 3]; 4];

    let p: &mut [i32; 3] = &mut a[1];

    *p[2] = 99;

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    if (p as *const [i32; 3] as usize - a.as_ptr() as usize) / std::mem::size_of::<[i32; 3]>() != 1 {
        std::process::exit(2);
    }
}