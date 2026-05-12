fn main() {
    let mut a = [[0; 3]; 4];

    let p = &mut a[1];

    p[2] = 99;

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    let p_offset = p as *const _ as usize - a.as_ptr() as usize;
    let element_size = std::mem::size_of::<[i32; 3]>();
    if p_offset / element_size != 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}