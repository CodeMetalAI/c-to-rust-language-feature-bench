fn main() {
    let p: [i32; 3] = [10, 20, 30];
    if p[0]!= 10 {
        std::process::exit(1);
    }
    if p[1]!= 20 {
        std::process::exit(2);
    }
    if p[2]!= 30 {
        std::process::exit(3);
    }

    let q = (7, 11);
    if q.0!= 7 {
        std::process::exit(4);
    }
    if q.1!= 11 {
        std::process::exit(5);
    }

    if [1, 2, 3][1]!= 2 {
        std::process::exit(6);
    }

    std::process::exit(0);
}