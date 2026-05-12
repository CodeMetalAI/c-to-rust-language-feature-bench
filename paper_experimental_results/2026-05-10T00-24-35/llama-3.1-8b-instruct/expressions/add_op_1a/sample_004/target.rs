fn main() {
    let mut a = [[0; 3]; 4];

    let p = &mut a[1];

    *p[2] = 99;

    if a[1][2]!= 99 {
        return 1;
    }

    let n = (p as *const _ as usize) - a as *const _;
    if n!= 1 {
        return 2;
    }

    assert_eq!(n, 1);
    assert_eq!(a[1][2], 99);
    std::process::exit(0);
}