use std::process::exit;

fn main() {
    let mut n_alltypes: i32 = 0;
    let mut ni_type: i32 = 0;
    let mut nf_type: i32 = 0;
    let mut nf_doublenode: f64 = 0.0;

    nf_type = 1;
    n_alltypes = nf_type;
    ni_type = nf_type;

    nf_doublenode = 3.14;

    if n_alltypes != 1 {
        exit(1);
    }
    if ni_type != 1 {
        exit(1);
    }
    if nf_type != 1 {
        exit(2);
    }
    if nf_doublenode != 3.14 {
        exit(3);
    }

    exit(0);
}