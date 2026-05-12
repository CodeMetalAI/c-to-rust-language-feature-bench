fn main() {
    let mut a = [[0; 3]; 4];

    let p: &mut [i32; 3] = &mut a[1];

    p[2] = 99;

    if a[1][2]!= 99 {
        return 1;
    }

    if (p as *const _ as usize - a.as_ptr() as usize)!= 1 {
        return 2;
    }

    assert_eq!(a[1][2], 99);
    assert_eq!((p as *const _ as usize - a.as_ptr() as usize), 1);
}