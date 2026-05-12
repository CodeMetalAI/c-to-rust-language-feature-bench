fn main() {
    let a = "abc";
    let b = "abc";

    if a.chars().next().unwrap() != 'a' || a.chars().nth(1).unwrap() != 'b' || a.chars().nth(2).unwrap() != 'c' || a.chars().last().unwrap() != '\0'
        || b.chars().next().unwrap() != 'a' || b.chars().nth(1).unwrap() != 'b' || b.chars().nth(2).unwrap() != 'c' || b.chars().last().unwrap() != '\0'
    {
        println!("Error in string a or b");
        return;
    }

    if a == b {
        println!("a and b are the same");
    } else {
        println!("a and b are not the same");
    }
}