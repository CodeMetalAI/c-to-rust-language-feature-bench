fn f(x: i32) -> i32 { x }

fn main() {
    let p0: usize = 0;
    let v0: usize = 0;
    let p1: usize = v0;

    let fp0: usize = 0;
    let fp1: usize = 0;
    let fp2: usize = v0;

    if p0 != p1 {
        std::process::exit(1);
    }
    if p0 != 0 {
        std::process::exit(2);
    }

    if v0 != 0 {
        std::process::exit(3);
    }
    if v0 != p0 {
        std::process::exit(4);
    }

    if fp0 != fp1 {
        std::process::exit(5);
    }
    if fp1 != fp2 {
        std::process::exit(6);
    }
    if fp0 != 0 {
        std::process::exit(7);
    }

    if p0 != fp0 {
        std::process::exit(8);
    }

    std::process::exit(0);
}