fn main() {
    let x = 1 - 1;
    let y = |a| a;

    if x != 0 {
        return 1;
    }
    if y(3) != 3 {
        return 2;
    }

    return 0;
}