fn main() -> i32 {
    let mut x = 41;

    let old = x;
    x += 1;
    if old != 41 {
        return 1;
    }
    if x != 42 {
        return 2;
    }

    let old = x;
    x += 1;
    if old != 42 {
        return 3;
    }
    if x != 43 {
        return 4;
    }

    x = 100;
    let y = x;
    x += 1;
    if y != 100 {
        return 5;
    }
    if x != 101 {
        return 6;
    }

    0
}