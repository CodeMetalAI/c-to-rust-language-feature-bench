fn main() {
    let v = ((10) + 2);
    let u = (((3, 4)) + 2);
    let w = F(0);

    let i: [i32; 4] = [1, 23, 4, 5];
    let c: [[char; 6]; 2] = [
        ['h', 'e', 'l', 'l', 'o', '\0'],
        ['\0', '\0', '\0', '\0', '\0', '\0'],
    ];

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

    if c[0][0] != 'h' {
        return;
    }
    if c[0][1] != 'e' {
        return;
    }
    if c[0][2] != 'l' {
        return;
    }
    if c[0][3] != 'l' {
        return;
    }
    if c[0][4] != 'o' {
        return;
    }
    if c[0][5] != '\0' {
        return;
    }

    if c[1][0] != '\0' {
        return;
    }

    std::process::exit(0);
}

fn F(a: i32) -> i32 {
    a + 2
}