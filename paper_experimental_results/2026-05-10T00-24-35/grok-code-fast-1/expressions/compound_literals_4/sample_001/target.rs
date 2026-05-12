use std::process;

fn main() {
    let p: &[f32] = &[1.0f32, 10.0f32, 100.0f32, 1000.0f32, 10000.0f32, 100000.0f32, 1000000.0f32];

    if p[0] != 1.0f32 {
        process::exit(1);
    }
    if p[1] != 10.0f32 {
        process::exit(2);
    }
    if p[2] != 100.0f32 {
        process::exit(3);
    }
    if p[3] != 1000.0f32 {
        process::exit(4);
    }
    if p[4] != 10000.0f32 {
        process::exit(5);
    }
    if p[5] != 100000.0f32 {
        process::exit(6);
    }
    if p[6] != 1000000.0f32 {
        process::exit(7);
    }

    process::exit(0);
}