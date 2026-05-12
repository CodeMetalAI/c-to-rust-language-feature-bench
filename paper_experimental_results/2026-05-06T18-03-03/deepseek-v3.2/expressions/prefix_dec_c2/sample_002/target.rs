fn main() {
    let mut x = 5;

    let y = {
        x -= 1;
        x
    };
    if y != 4 {
        return;
    }
    if x != 4 {
        return;
    }

    x = 10;
    let y = {
        x -= 1;
        x
    };
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