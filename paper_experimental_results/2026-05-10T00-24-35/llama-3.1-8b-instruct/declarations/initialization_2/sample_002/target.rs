fn sum3(a: [i32; 3]) -> i32 {
    a[0] + a[1] + a[2]
}

fn main() {
    let mut x = [1, 3, 5];

    if x.len() != 3 {
        eprintln!("Error: array length mismatch");
        std::process::exit(1);
    }

    if x[0] != 1 || x[1] != 3 || x[2] != 5 {
        eprintln!("Error: array value mismatch");
        std::process::exit(2);
    }

    if sum3(x) != 9 {
        eprintln!("Error: sum mismatch");
        std::process::exit(3);
    }

    {
        let p = &x;
        if p[2] != 5 {
            eprintln!("Error: array value mismatch");
            std::process::exit(4);
        }
    }

    println!("Success");
    std::process::exit(0);
}