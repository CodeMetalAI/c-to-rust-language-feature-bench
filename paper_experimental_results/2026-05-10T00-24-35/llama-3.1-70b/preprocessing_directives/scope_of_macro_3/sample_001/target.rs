fn main() {
    let x = 3;
    let v = (10 + x);
    let u = (3) + x;
    let w = 0 + 2;

    let i = [1, 23, 4, 5];
    let c = [["hello".as_bytes(), &[b'\0']];

    if v != 12 {
        return 1;
    }
    if u != 6 {
        return 2;
    }
    if w != 2 {
        return 3;
    }

    if i[0] != 1 {
        return 4;
    }
    if i[1] != 23 {
        return 5;
    }
    if i[2] != 4 {
        return 6;
    }
    if i[3] != 5 {
        return 7;
    }

    if c[0][0] != b'h' {
        return 8;
    }
    if c[0][1] != b'e' {
        return 9;
    }
    if c[0][2] != b'l' {
        return 10;
    }
    if c[0][3] != b'l' {
        return 11;
    }
    if c[0][4] != b'o' {
        return 12;
    }
    if c[0][5] != b'\0' {
        return 13;
    }

    if c[1][0] != b'\0' {
        return 14;
    }

    return 0;
}