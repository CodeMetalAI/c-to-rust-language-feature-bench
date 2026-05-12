fn main() {
    let a = ["a", "b", "c"];
    let b = ["a", "b", "c"];

    if a.iter().chain(Some(&'\0')).zip("abc\0".chars()).any(|(a, b)| a!= b) {
        return 1;
    }

    if b.iter().chain(Some(&'\0')).zip("abc\0".chars()).any(|(a, b)| a!= b) {
        return 2;
    }

    if a.as_ptr() as *const _ as usize!= b.as_ptr() as *const _ as usize {
        return 3;
    }

    return 0;
}