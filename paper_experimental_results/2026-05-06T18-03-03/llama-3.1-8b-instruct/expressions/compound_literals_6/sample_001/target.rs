fn main() {
    let a = "abc";
    let b = "abc";

    if a.chars().next().unwrap()!= 'a' || a.chars().nth(1).unwrap()!= 'b' || a.chars().nth(2).unwrap()!= 'c' || a.chars().last().unwrap()!= '\0'
    {
        return 1;
    }
    if b.chars().next().unwrap()!= 'a' || b.chars().nth(1).unwrap()!= 'b' || b.chars().nth(2).unwrap()!= 'c' || b.chars().last().unwrap()!= '\0'
    {
        return 2;
    }

    if a == b {
        return 3;
    }

    return 0;
}