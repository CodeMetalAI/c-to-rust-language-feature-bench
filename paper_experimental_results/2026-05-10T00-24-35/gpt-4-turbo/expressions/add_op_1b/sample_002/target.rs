fn main() {
    let mut a = [[0; 3]; 4];
    
    {
        let p = &mut a[1..];
        p[0][2] = 99;
    }

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    let p = &a[1..];
    if (p.as_ptr() as usize).wrapping_sub(a.as_ptr() as usize) / std::mem::size_of::<[i32; 3]>() != 1 {
        std::process::exit(2);
    }
}