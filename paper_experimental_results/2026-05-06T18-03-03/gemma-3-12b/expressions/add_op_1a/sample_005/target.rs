fn main() {
    let n = 4;
    let m = 3;
    let mut a = vec![vec![0; m]; n];

    let mut p = a.as_mut_slice();

    unsafe {
        p = &mut p[1];
        (*p)[2] = 99;
    }

    if a[1][2] != 99 {
        return 1;
    }

    let n = {
        let ptr_diff = (p as *mut Vec<[i32; 3]>) - (a.as_mut_slice() as *mut Vec<[i32; 3]>);
        ptr_diff as usize / m
    };

    if n != 1 {
        return 2;
    }

    0
}