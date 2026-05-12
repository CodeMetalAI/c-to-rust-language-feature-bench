fn main() {
    let a = ["a", "b", "c"];
    let b = "abc";

    if a[0] != 'a' || a[1] != 'b' || a[2] != 'c' || a.len() != 3 {
        return 1;
    }
    if b[0] != 'a' || b[1] != 'b' || b[2] != 'c' || b.len() != 3 {
        return 2;
    }

    if a.len() == b.len() && a == b {
        return 3;
    }

    return 0;
}