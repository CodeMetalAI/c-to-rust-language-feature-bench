fn main() {
    let s = std::mem::size_of::<i32>();
    let offset_i = std::mem::align_of::<i32>();
    let offset_a = s - offset_i;

    let p = Box::new(std::mem::ManuallyDrop::new([0; 4]));
    let i = &mut p[0];
    let a = &mut p[1..];

    if s!= offset_i {
        return 1;
    }

    if s!= offset_a {
        return 2;
    }

    *i = 7;
    a[0] = 11;
    a[3] = 22;

    if *i!= 7 || a[0]!= 11 || a[3]!= 22 {
        return 4;
    }

    std::mem::drop(p);
    std::process::exit(0);
}