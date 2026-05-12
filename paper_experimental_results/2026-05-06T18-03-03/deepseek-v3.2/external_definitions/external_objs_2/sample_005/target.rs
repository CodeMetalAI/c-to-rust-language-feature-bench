static mut I: [i32; 1] = [0];

fn main() {
    let result = unsafe {
        if I[0] != 0 {
            return;
        }
        I[0] = -1;
        if I[0] != -1 {
            return;
        }
        0
    };
    std::process::exit(result);
}