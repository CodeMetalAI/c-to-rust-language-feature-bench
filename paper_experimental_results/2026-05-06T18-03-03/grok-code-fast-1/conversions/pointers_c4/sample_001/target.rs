fn main() {
    let p0: Option<Box<i32>> = None;
    let v0: Option<Box<()>> = None;
    let p1: Option<Box<i32>> = None;

    let fp0: Option<fn(i32) -> i32> = None;
    let fp1: Option<fn(i32) -> i32> = None;
    let fp2: Option<fn(i32) -> i32> = None;

    if p0 != p1 {
        std::process::exit(1);
    }
    if p0 != None {
        std::process::exit(2);
    }

    if v0 != None {
        std::process::exit(3);
    }
    if !(v0.is_none() && p0.is_none()) {
        std::process::exit(4);
    }

    if fp0 != fp1 {
        std::process::exit(5);
    }
    if fp1 != fp2 {
        std::process::exit(6);
    }
    if fp0 != None {
        std::process::exit(7);
    }

    if !(p0.is_none() && fp0.is_none()) {
        std::process::exit(8);
    }

    std::process::exit(0);
}