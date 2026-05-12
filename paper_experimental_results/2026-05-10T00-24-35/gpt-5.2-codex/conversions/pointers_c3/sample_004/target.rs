fn f(x: i32) -> i32 {
    x + 1
}

fn main() {
    let _x: i32 = 1;
    let px: usize = 1;
    let pf: usize = 2;

    let n1: usize = 0;
    let v0: usize = 0;
    let n2: usize = v0;

    let nf1: usize = 0;
    let vf0: usize = 0;
    let nf2: usize = vf0;

    if n1 != 0 {
        std::process::exit(1);
    }
    if n2 != 0 {
        std::process::exit(2);
    }
    if v0 != 0 {
        std::process::exit(3);
    }

    if n1 == px {
        std::process::exit(4);
    }
    if n2 == px {
        std::process::exit(5);
    }

    if nf1 != 0 {
        std::process::exit(6);
    }
    if nf2 != 0 {
        std::process::exit(7);
    }
    if nf1 == pf {
        std::process::exit(8);
    }
    if nf2 == pf {
        std::process::exit(9);
    }

    let vx: usize = px;
    let vf: usize = pf;

    if vx == 0 {
        std::process::exit(10);
    }
    if vf == 0 {
        std::process::exit(11);
    }
    if vx == v0 {
        std::process::exit(12);
    }
    if vf == vf0 {
        std::process::exit(13);
    }

    std::process::exit(0);
}