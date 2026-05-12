fn main() {
    const X: i32 = 2;
    const F: fn(i32) -> i32 = |a| a + X;

    let v = F(10);
    let u = F(3);
    let w = F(0);

    let i: [i32; 4] = [1, 23, 4, 5];

    let c: [&[u8]; 2] = [b"hello", b""];

    if v!= 12 {
        return 1;
    }
    if u!= 5 {
        return 2;
    }
    if w!= 2 {
        return 3;
    }

    if i[0]!= 1 {
        return 4;
    }
    if i[1]!= 23 {
        return 5;
    }
    if i[2]!= 4 {
        return 6;
    }
    if i[3]!= 5 {
        return 7;
    }

    if c[0][0]!= b'h' {
        return 8;
    }
    if c[0][1]!= b'e' {
        return 9;
    }
    if c[0][2]!= b'l' {
        return 10;
    }
    if c[0][3]!= b'l' {
        return 11;
    }
    if c[0][4]!= b'o' {
        return 12;
    }
    if c[0][5]!= 0 {
        return 13;
    }

    if c[1][0]!= 0 {
        return 14;
    }

    return 0;
}