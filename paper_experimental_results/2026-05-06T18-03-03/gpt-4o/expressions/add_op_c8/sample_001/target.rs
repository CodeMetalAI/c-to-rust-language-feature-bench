fn main() -> i32 {
    let a = [10, 20, 30, 40, 50];

    let p0 = 0;
    let p2 = p0 + 2;
    let p4 = p2 + 2;
    let p_last = 4;
    let p_end = p_last + 1;

    if a[p2] != 30 {
        return 1;
    }
    if a[p4] != 50 {
        return 2;
    }

    let p_back = p4 - 3;
    if a[p_back] != 20 {
        return 3;
    }

    let p_from_end = p_end - 1;
    if p_from_end != p_last {
        return 4;
    }
    if a[p_from_end] != 50 {
        return 5;
    }

    if (p0 + 5) != p_end {
        return 6;
    }

    0
}