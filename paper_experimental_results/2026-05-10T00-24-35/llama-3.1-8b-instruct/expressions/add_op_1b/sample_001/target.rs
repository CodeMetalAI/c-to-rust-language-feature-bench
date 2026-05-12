fn main() {
    let mut a = [[0; 3]; 4];

    let p = &mut a[1];

    *p = [0, 0, 99];

    if a[1][2] != 99 {
        println!("a[1][2] is not 99");
        return;
    }

    if p - &a != 1 {
        println!("p - a is not 1");
        return;
    }

    println!("Program executed successfully");
}