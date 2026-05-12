fn main() -> i32 {
    let v = 12;
    let u = 6;
    let w = 2;
    let i: [i32; 5] = [1, 23, 4, 5, 0];
    let c: [[char; 6]; 2] = [
        ['h', 'e', 'l', 'l', 'o', '\0'],
        ['\0', '\0', '\0', '\0', '\0', '\0'],
    ];

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

    if c[0][0] != 'h' {
        return 8;
    }
    if c[0][1] != 'e' {
        return 9;
    }
    if c[0][2] != 'l' {
        return 10;
    }
    if c[0][3] != 'l' {
        return 11;
    }
    if c[0][4] != 'o' {
        return 12;
    }
    if c[0][5] != '\0' {
        return 13;
    }

    if c[1][0] != '\0' {
        return 14;
    }

    0
}