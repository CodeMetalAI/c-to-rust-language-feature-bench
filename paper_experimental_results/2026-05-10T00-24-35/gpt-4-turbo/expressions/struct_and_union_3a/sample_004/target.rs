fn main() {
    let mut u = U::default();

    // Setting `type` and `doublenode` in the `nf` field
    if let U::Nf(ref mut nf) = u {
        nf.type_ = 1;
        nf.doublenode = 3.14;
    }

    // Check conditions to match the behavior of the original C++ program
    // Due to the union, accessing any field should reflect the last written values
    match u {
        U::N(ref n) if n.alltypes == 1 => (),
        _ => return std::process::exit(1),
    };

    if let U::Ni(ni) = u {
        if ni.type_ != 1 {
            return std::process::exit(1);
        }
    }

    match u {
        U::Nf(nf) if nf.type_ == 1 && (nf.doublenode - 3.14).abs() < f64::EPSILON => (),
        _ => return std::process::exit(3),
    };
}

#[derive(Debug, Clone, Copy)]
enum U {
    N(N),
    Ni(Ni),
    Nf(Nf),
}

impl Default for U {
    fn default() -> Self {
        U::Nf(Nf { type_: 0, doublenode: 0.0 })
    }
}

#[derive(Debug, Clone, Copy)]
struct N {
    alltypes: i32,
}

#[derive(Debug, Clone, Copy)]
struct Ni {
    type_: i32,
    intnode: i32,
}

#[derive(Debug, Clone, Copy)]
struct Nf {
    type_: i32,
    doublenode: f64,
}