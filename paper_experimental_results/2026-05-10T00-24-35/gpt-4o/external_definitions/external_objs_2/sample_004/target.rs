static mut I: [i32; 1] = [0; 1];

fn main() {
    // Safe block to handle mutable static variable
    let result = unsafe {
        if I[0] != 0 {
            return 1;
        }
        I[0] = 7;
        if I[0] != 7 {
            return 2;
        }
        0
    };

    std::process::exit(result);
}