fn sum(arr: &[i32]) -> i32 {
    let mut s = 0;
    for x in arr {
        s += x;
    }
    s
}

fn main() {
    let a = [1, 2];
    let b = [3, 4, 5];

    if a.len() != 2 {
        println!("Error: Length of array a is not 2");
        std::process::exit(1);
    }
    if b.len() != 3 {
        println!("Error: Length of array b is not 3");
        std::process::exit(2);
    }

    if sum(&a) != 3 {
        println!("Error: Sum of array a is not 3");
        std::process::exit(3);
    }
    if sum(&b) != 12 {
        println!("Error: Sum of array b is not 12");
        std::process::exit(4);
    }

    {
        let pa = &a;
        let pb = &b;
        if pa[1] != 2 {
            println!("Error: pa[1] is not 2");
            std::process::exit(5);
        }
        if pb[2] != 5 {
            println!("Error: pb[2] is not 5");
            std::process::exit(6);
        }
    }

    println!("All tests passed");
    std::process::exit(0);
}