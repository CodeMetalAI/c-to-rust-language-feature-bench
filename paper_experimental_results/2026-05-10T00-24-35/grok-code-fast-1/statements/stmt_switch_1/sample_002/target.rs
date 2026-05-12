fn main() {
    let expr = 0;
    let mut i = 4;
    let exit_code = match expr {
        0 => {
            i = 17;
            if i == 17 { 0 } else { 1 }
        }
        _ => {
            // In C, if no case matches, behavior is undefined, but since expr is 0, this won't be reached.
            // To preserve potential fallthrough, we can return some value, but for this program, it's not hit.
            unreachable!()
        }
    };
    std::process::exit(exit_code);
}