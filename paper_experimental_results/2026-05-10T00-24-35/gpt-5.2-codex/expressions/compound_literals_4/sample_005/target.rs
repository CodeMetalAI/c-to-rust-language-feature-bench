use std::process::exit;

fn main() {
    let arr: [f32; 7] = [1e0f32, 1e1f32, 1e2f32, 1e3f32, 1e4f32, 1e5f32, 1e6f32];
    let p: &[f32] = &arr;

    if p[0] != 1e0f32 {
        exit(1);
    }
    if p[1] != 1e1f32 {
        exit(2);
    }
    if p[2] != 1e2f32 {
        exit(3);
    }
    if p[3] != 1e3f32 {
        exit(4);
    }
    if p[4] != 1e4f32 {
        exit(5);
    }
    if p[5] != 1e5f32 {
        exit(6);
    }
    if p[6] != 1e6f32 {
        exit(7);
    }

    exit(0);
}