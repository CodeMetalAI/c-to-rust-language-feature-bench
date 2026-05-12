fn main() {
    let i = 3;
    let j = 5;
    if i.max(j) != 5 {
        std::process::exit(1);
    }

    let x = 2.5;
    let y = 1.5;
    if x.max(y) != 2.5 {
        std::process::exit(2);
    }

    let mut k = 1;
    let r = {
        let temp_k = k;
        k += 1;
        temp_k.max(2)
    };
    if r != 2 {
        std::process::exit(3);
    }
    if k != 2 {
        std::process::exit(4);
    }

    let mut m = 3;
    let mut n = 1;
    let r = {
        let temp_m = m;
        m += 1;
        let temp_n = n;
        n += 1;
        temp_m.max(temp_n)
    };
    if r != 3 {
        std::process::exit(5);
    }
    if m != 4 {
        std::process::exit(6);
    }
    if n != 2 {
        std::process::exit(7);
    }
}