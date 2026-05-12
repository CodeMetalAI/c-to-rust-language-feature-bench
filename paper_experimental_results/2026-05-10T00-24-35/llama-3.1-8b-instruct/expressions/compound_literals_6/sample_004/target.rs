fn main() {
    let a = "abc";
    let b = "abc";

    if a.chars().zip(b.chars()).any(|(c, d)| c!= d) {
        return 1;
    }

    if a == b {
        return 3;
    }

    return 0;
}