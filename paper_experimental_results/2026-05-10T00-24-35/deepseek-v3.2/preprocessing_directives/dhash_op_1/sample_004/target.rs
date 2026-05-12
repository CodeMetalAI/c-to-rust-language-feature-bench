// Converted from the C/C++ program to Rust

fn main() {
    let p = ['x', '#', '#', 'y'];
    
    // Check each character and return specific exit codes
    if p[0] != 'x' {
        std::process::exit(1);
    }
    if p[1] != '#' {
        std::process::exit(2);
    }
    if p[2] != '#' {
        std::process::exit(3);
    }
    if p[3] != 'y' {
        std::process::exit(4);
    }
    
    // Exit with code 0 if all checks pass
    std::process::exit(0);
}