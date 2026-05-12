fn sum(p: &[i32], n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        s += p[i];
    }
    s
}

fn main() {
    let a: [i32; 2] = [1, 2];
    let b: [i32; 3] = [3, 4, 5];

    if a.len()!= 2 || b.len()!= 3 {
        println!("Error: array sizes mismatch");
        std::process::exit(1);
    }

    if sum(&a, a.len())!= 3 {
        println!("Error: sum of a is incorrect");
        std::process::exit(2);
    }
    if sum(&b, b.len())!= 12 {
        println!("Error: sum of b is incorrect");
        std::process::exit(3);
    }

    {
        let pa = &a;
        let pb = &b;
        if pa[1]!= 2 {
            println!("Error: element of a is incorrect");
            std::process::exit(4);
        }
        if pb[2]!= 5 {
            println!("Error: element of b is incorrect");
            std::process::exit(5);
        }
    }

    println!("All tests passed");
    std::process::exit(0);
}