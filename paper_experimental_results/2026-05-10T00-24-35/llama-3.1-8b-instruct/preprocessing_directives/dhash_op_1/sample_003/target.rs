fn main() {
    let p = format!("x##{ }y");

    if p.len()!= 7 {
        println!("Error: sizeof(p)!= 7");
        std::process::exit(1);
    }

    if p.chars().nth(0).unwrap()!= 'x' {
        println!("Error: p[0]!= 'x'");
        std::process::exit(2);
    }

    if p.chars().nth(1).unwrap()!= ' ' {
        println!("Error: p[1]!= ' '");
        std::process::exit(3);
    }

    if p.chars().nth(2).unwrap()!= '#' {
        println!("Error: p[2]!= '#'");
        std::process::exit(4);
    }

    if p.chars().nth(3).unwrap()!= '#' {
        println!("Error: p[3]!= '#'");
        std::process::exit(5);
    }

    if p.chars().nth(4).unwrap()!= ' ' {
        println!("Error: p[4]!= ' '");
        std::process::exit(6);
    }

    if p.chars().nth(5).unwrap()!= 'y' {
        println!("Error: p[5]!= 'y'");
        std::process::exit(7);
    }

    if p.chars().nth(6).unwrap()!= '\0' {
        println!("Error: p[6]!= '\\0'");
        std::process::exit(8);
    }

    println!("All checks passed.");
    std::process::exit(0);
}