fn main() {
    let mut memory = vec![b'A']; // memory[0] is c
    let mut p_index: usize = 0; // Simulate setting p to point to c (index 0)
    memory[p_index] = 0; // *p = 0;
    let c = memory[0];
    if c != b'A' {
        std::process::exit(1);
    } else {
        std::process::exit(0);
    }
}