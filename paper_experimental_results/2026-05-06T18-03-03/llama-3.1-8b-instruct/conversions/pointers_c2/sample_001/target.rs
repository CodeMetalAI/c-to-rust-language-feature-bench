fn main() {
    let mut x = 17;
    let p = &x;

    let pc = p;
    let pv = p;
    let pcv = p;

    if pc != p {
        return 1;
    }
    if pv != p {
        return 2;
    }
    if pcv != p {
        return 3;
    }

    if *pc != 17 {
        return 4;
    }
    if *pv != 17 {
        return 5;
    }
    if *pcv != 17 {
        return 6;
    }

    0
}