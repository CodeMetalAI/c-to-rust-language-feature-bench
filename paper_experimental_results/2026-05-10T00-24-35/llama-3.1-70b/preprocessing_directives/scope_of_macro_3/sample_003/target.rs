fn main() {
    const X: i32 = 3;
    let v = 10 + X;
    let u = 3 + X;
    let w = 0 + 2;

    let i = [1, 23, 4, 5];
    let c = [["h", "e", "l", "l", "o", '\0'], ["\0"]];

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

    if c[0][0]!= 'h' {
        return;
    }
    if c[0][1]!= 'e' {
        return;
    }
    if c[0][2]!= 'l' {
        return;
    }
    if c[0][3]!= 'l' {
        return;
    }
    if c[0][4]!= 'o' {
        return;
    }
    if c[0][5]!= '\0' {
        return;
    }

    if c[1][0]!= '\0' {
        return;
    }
    std::process::exit(0);
}