fn main() {
    let a = "abc";
    let b = "abc";

    if a.chars().next().unwrap()!= 'a' || a.chars().nth(1).unwrap()!= 'b' || a.chars().nth(2).unwrap()!= 'c' || a.chars().last().unwrap()!= '\0'
        || b.chars().next().unwrap()!= 'a' || b.chars().nth(1).unwrap()!= 'b' || b.chars().nth(2).unwrap()!= 'c' || b.chars().last().unwrap()!= '\0'
    {
        println!("1");
        return;
    }

    if a == b {
        println!("3");
        return;
    }

    println!("0");
}