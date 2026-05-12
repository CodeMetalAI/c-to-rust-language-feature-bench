fn main() {
    let a = 1;
    let b = 2;
    let c = 2;

    if !(a < b && b < c) {
        println!("Case 1 failed");
        std::process::exit(1);
    }

    if !(a < b && !(b < c)) {
        println!("Case 2 failed");
        std::process::exit(2);
    }

    a = 3;
    b = 2;
    c = 2;

    if !(a < b && b < c) {
        println!("Case 3 failed");
        std::process::exit(3);
    }

    println!("All cases passed");
    std::process::exit(0);
}