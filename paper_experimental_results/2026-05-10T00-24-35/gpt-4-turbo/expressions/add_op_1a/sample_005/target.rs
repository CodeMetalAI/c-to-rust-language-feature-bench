fn main() {
    let mut a = vec![
        vec![0; 3],
        vec![0; 3],
        vec![0; 3],
        vec![0; 3],
    ];

    {
        let mut p = &mut a[1..];
        p[0][2] = 99;
    }

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    let p = &a[1..];
    let n = p.as_ptr() as usize - a.as_ptr() as usize;

    if n != 1 {
        std::process::exit(2);
    }
}