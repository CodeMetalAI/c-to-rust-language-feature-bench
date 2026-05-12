fn main() {
    let arr = [1e0_f32, 1e1_f32, 1e2_f32, 1e3_f32, 1e4_f32, 1e5_f32, 1e6_f32];

    if arr[0] != 1e0_f32 {
        std::process::exit(1);
    }
    if arr[1] != 1e1_f32 {
        std::process::exit(2);
    }
    if arr[2] != 1e2_f32 {
        std::process::exit(3);
    }
    if arr[3] != 1e3_f32 {
        std::process::exit(4);
    }
    if arr[4] != 1e4_f32 {
        std::process::exit(5);
    }
    if arr[5] != 1e5_f32 {
        std::process::exit(6);
    }
    if arr[6] != 1e6_f32 {
        std::process::exit(7);
    }

    std::process::exit(0);
}