fn main() {
    // Corresponds to pointer equality for the same variable
    let x = 0;
    let px1 = 0usize;
    let px2 = 0usize;
    if px1 != px2 {
        std::process::exit(1);
    }

    // Array end pointers
    let _arr = [1, 2, 3];
    let p_end1 = 3usize;
    let p_end2 = 3usize;
    if p_end1 != p_end2 {
        std::process::exit(2);
    }

    // Pointer comparisons within array
    let p0 = 0usize;
    let p2 = 2usize;
    if !(p2 > p0) {
        std::process::exit(3);
    }
    if !(p0 < p2) {
        std::process::exit(4);
    }

    let q_last = 2usize;
    let q1 = q_last + 1;
    if !(q1 > p0) {
        std::process::exit(5);
    }

    // Struct field order comparison
    let sa = 0usize;
    let sb = 1usize;
    if !(sb > sa) {
        std::process::exit(6);
    }

    // Union fields share the same address
    let ui = 0usize;
    let ud = 0usize;
    if ui != ud {
        std::process::exit(7);
    }

    std::process::exit(0);
}