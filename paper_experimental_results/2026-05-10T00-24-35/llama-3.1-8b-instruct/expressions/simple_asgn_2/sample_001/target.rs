fn main() {
    let mut c: i8 = 0;
    let mut i: i32 = 100;
    let mut l: i64 = 0;

    l = (c = i) as i64;

    if c!= i as i8 {
        println!("1");
        return;
    }
    if l!= c as i64 {
        println!("2");
        return;
    }
    if l!= i as i64 {
        println!("3");
        return;
    }

    println!("0");
}