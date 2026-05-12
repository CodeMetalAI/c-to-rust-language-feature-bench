fn main() {
    let mut sink: u8 = 0;
    let n: i32 = 7;
    let mut b: [i32; n as usize] = [0; n as usize];
    let mut a: [i32; (n - 1) as usize] = [0; (n - 1) as usize];

    for i in 0..n {
        b[i as usize] = 1000 + i;
    }

    for i in 1..n {
        a[i as usize - 1] = b[i as usize];
    }

    if a[0]!= 1001 {
        sink |= 1;
    }
    if a[(n - 2) as usize]!= 1000 + (n - 1) {
        sink |= 2;
    }

    if b.len()!= n {
        sink |= 4;
    }

    if b.len()!= (n - 1) {
        sink |= 8;
    }

    if sink!= 0 {
        return 1;
    }
    return 0;
}