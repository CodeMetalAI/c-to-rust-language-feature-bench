fn main() {
    let x = 3;
    let f = |a| a + x;
    let x = 2;

    let t = |a| a;
    let g = f;
    let q = |x| x;
    let p = || 0;
    let r = |x, y| format!("{}{}", x, y);
    let str = |x| format!("{}", x);

    let v = f(10);
    let u = f(3 + 4);
    let w = t(g)(0);

    let mut i = [0; 4];
    i[0] = q(1);
    i[1] = r(2, 3).parse::<i32>().unwrap();
    i[2] = r(4, "").parse::<i32>().unwrap();
    i[3] = r("", 5).parse::<i32>().unwrap();

    let c = [
        str("hello").as_bytes().to_vec(),
        str("").as_bytes().to_vec(),
    ];

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

    if c[0][0]!= b'h' {
        return;
    }
    if c[0][1]!= b'e' {
        return;
    }
    if c[0][2]!= b'l' {
        return;
    }
    if c[0][3]!= b'l' {
        return;
    }
    if c[0][4]!= b'o' {
        return;
    }
    if c[0][5]!= 0 {
        return;
    }

    if c[1][0]!= 0 {
        return;
    }

    println!("Success");
}