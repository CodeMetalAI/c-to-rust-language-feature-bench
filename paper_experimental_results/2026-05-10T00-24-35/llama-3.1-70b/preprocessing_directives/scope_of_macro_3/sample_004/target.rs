fn main() {
    let x = 3;
    let v = 10 + x;
    let x = 2;
    let u = 4 + x;
    let w = 0 + x;

    let i = [1, 23, 4, 5];
    let c = [["hello".as_bytes(), "".as_bytes()]];

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
    if c[0][5]!= b'\0' {
        return;
    }

    if c[1][0]!= b'\0' {
        return;
    }

    println!();
}