fn sum(p: &[i32], n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        s += p[i];
    }
    s
}

fn main() {
    let a = [1, 2];
    let b = [3, 4, 5];

    if a.len() != 2 {
        println!("Error: a has incorrect length");
        std::process::exit(1);
    }
    if b.len() != 3 {
        println!("Error: b has incorrect length");
        std::process::exit(2);
    }

    if sum(&a, a.len()) != 3 {
        println!("Error: sum of a is incorrect");
        std::process::exit(3);
    }
    if sum(&b, b.len()) != 12 {
        println!("Error: sum of b is incorrect");
        std::process::exit(4);
    }

    {
        let pa = &a;
        let pb = &b;
        if pa[1] != 2 {
            println!("Error: pa[1] is incorrect");
            std::process::exit(5);
        }
        if pb[2] != 5 {
            println!("Error: pb[2] is incorrect");
            std::process::exit(6);
        }
    }

    println!("All tests passed");
    std::process::exit(0);
}