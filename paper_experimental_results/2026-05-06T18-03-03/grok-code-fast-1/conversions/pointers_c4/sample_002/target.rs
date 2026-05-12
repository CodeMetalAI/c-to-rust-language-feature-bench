use std::process;

fn main() {
    let p0: usize = 0;
    let v0: usize = 0;
    let p1: usize = v0;

    let fp0: usize = 0;
    let fp1: usize = 0;
    let fp2: usize = v0;

    if p0 != p1 {
        process::exit(1);
    }
    if p0 != 0 {
        process::exit(2);
    }

    if v0 != 0 {
        process::exit(3);
    }
    if v0 != p0 {
        process::exit(4);
    }

    if fp0 != fp1 {
        process::exit(5);
    }
    if fp1 != fp2 {
        process::exit(6);
    }
    if fp0 != 0 {
        process::exit(7);
    }

    if p0 != fp0 {
        process::exit(8);
    }

    process::exit(0);
}