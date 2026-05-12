fn main() {
    let p: &[f32] = &[1e0_f32, 1e1_f32, 1e2_f32, 1e3_f32, 1e4_f32, 1e5_f32, 1e6_f32];

    if p[0] != 1e0_f32 {
        std::process::exit(1);
    }
    if p[1] != 1e1_f32 {
        std::process::exit(2);
    }
    if p[2] != 1e2_f32 {
        std::process::exit(3);
    }
    if p[3] != 1e3_f32 {
        std::process::exit(4);
    }
    if p[4] != 1e4_f32 {
        std::process::exit(5);
    }
    if p[5] != 1e5_f32 {
        std::process::exit(6);
    }
    if p[6] != 1e6_f32 {
        std::process::exit(7);
    }

    std::process::exit(0);
}