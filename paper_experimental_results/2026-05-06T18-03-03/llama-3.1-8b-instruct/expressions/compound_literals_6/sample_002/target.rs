fn main() {
    let a = "abc";
    let b = "abc";

    if a.chars().zip(a.chars()).any(|(c, d)| c != d) {
        return 1;
    }
    if b.chars().zip(b.chars()).any(|(c, d)| c != d) {
        return 2;
    }

    if a == b {
        return 3;
    }

    return 0;
}