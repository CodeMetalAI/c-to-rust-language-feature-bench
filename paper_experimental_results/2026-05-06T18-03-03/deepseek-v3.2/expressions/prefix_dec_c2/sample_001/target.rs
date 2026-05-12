fn main() {
    let mut x = 5;

    x -= 1;
    let y = x;
    if y != 4 {
        return;
    }
    if x != 4 {
        return;
    }

    x = 10;
    x -= 1;
    let y = x;
    let mut z = 10;
    z -= 1;
    if y != 9 {
        return;
    }
    if x != 9 {
        return;
    }
    if z != 9 {
        return;
    }
}