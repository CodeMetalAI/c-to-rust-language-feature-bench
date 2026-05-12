fn main() {
    let x = 3;
    let f = |a| a + x;
    let v = f(10);
    let u = f(3 + 4);
    let w = f(0);

    let mut i = [0; 4];
    i[0] = 1;
    i[1] = 23;
    i[2] = 4;
    i[3] = 5;

    let mut c = [[0u8; 6]; 2];
    c[0].copy_from_slice(b"hello");
    c[1].copy_from_slice(b"");

    if v != 12 {
        return;
    }
    if u != 6 {
        return;
    }
    if w != 2 {
        return;
    }

    if i[0] != 1 {
        return;
    }
    if i[1] != 23 {
        return;
    }
    if i[2] != 4 {
        return;
    }
    if i[3] != 5 {
        return;
    }

    if c[0][0] != b'h' {
        return;
    }
    if c[0][1] != b'e' {
        return;
    }
    if c[0][2] != b'l' {
        return;
    }
    if c[0][3] != b'l' {
        return;
    }
    if c[0][4] != b'o' {
        return;
    }
    if c[0][5] != 0 {
        return;
    }

    if c[1][0] != 0 {
        return;
    }

    std::process::exit(0);
}