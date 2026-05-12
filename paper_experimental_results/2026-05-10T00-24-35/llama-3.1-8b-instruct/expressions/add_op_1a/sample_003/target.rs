fn main() {
    let n = 4;
    let m = 3;
    let mut a = vec![vec![0; m]; n];

    let p = a.as_mut_ptr();

    unsafe {
        p.add(1).write(99);

        if a[1][2]!= 99 {
            return 1;
        }

        let n = p as *mut [usize].offset_from(a.as_mut_ptr());
        if n!= 1 {
            return 2;
        }
    }

    return 0;
}