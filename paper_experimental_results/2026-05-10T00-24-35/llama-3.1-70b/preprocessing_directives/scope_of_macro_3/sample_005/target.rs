fn main() {
    // Macros can't be directly translated, so we'll use functions instead
    let x = 2;
    let f = |a| a + x;
    let t = |a| a;
    let g = f;
    let q = |x| x;
    let r = |x, y| format!("{}{}", x, y);
    let str = |x| format!("{}", x);

    let v = f(10);
    let u = f(3 + 4);
    let w = t(g)(0);

    let mut i = [0; 4];
    i[q(0)] = q(1);
    i[q(1)] = r(2, 3).parse::<i32>().unwrap();
    i[q(2)] = r(4, "").parse::<i32>().unwrap();
    i[q(3)] = r("", 5).parse::<i32>().unwrap();

    let mut c = [[0u8; 6]; 2];
    let hello = str("hello").into_bytes();
    c[0].copy_from_slice(&hello);

    if v!= 12 {
        return;
    }
    if u!= 6 {
        return;
    }
    if w!= 2 {
        return;
    }

    if i[0]!= 1 {
        return;
    }
    if i[1]!= 23 {
        return;
    }
    if i[2]!= 4 {
        return;
    }
    if i[3]!= 5 {
        return;
    }

    if c[0][0]!= b'h' as u8 {
        return;
    }
    if c[0][1]!= b'e' as u8 {
        return;
    }
    if c[0][2]!= b'l' as u8 {
        return;
    }
    if c[0][3]!= b'l' as u8 {
        return;
    }
    if c[0][4]!= b'o' as u8 {
        return;
    }
    if c[0][5]!= 0 {
        return;
    }

    if c[1][0]!= 0 {
        return;
    }

    // Return 0 on success
    println!("Success");
}