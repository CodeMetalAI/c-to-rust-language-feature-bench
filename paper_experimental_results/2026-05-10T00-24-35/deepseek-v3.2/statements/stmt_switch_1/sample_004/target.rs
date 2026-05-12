fn main() {
    let expr = 0;
    let i: i32 = {
        // In C, the variable i is declared at switch scope but initialized after jumping to case
        // In Rust we need to handle this differently since there's no direct equivalent
        // The C code effectively does:
        // - Declare i (uninitialized)
        // - Jump to case 0 (skipping i = 4)
        // - Set i = 17
        // - Return based on i == 17
        // So we just initialize i to 17 directly
        17
    };
    
    if expr == 0 {
        std::process::exit(if i == 17 { 0 } else { 1 });
    }
}