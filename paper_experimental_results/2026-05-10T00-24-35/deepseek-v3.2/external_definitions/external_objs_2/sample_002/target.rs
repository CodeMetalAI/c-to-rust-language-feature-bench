static mut I: [i32; 1] = [0];

fn main() {
    unsafe {
        if I[0] != 0 {
            return;
        }
        I[0] = 7;
        if I[0] != 7 {
            return;
        }
    }
}