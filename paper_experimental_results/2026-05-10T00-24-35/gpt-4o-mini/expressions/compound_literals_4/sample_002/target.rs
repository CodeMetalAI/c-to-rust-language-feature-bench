fn main() -> i32 {
    let p: [f32; 7] = [1e0f32, 1e1f32, 1e2f32, 1e3f32, 1e4f32, 1e5f32, 1e6f32];

    if p[0] != 1e0f32 {
        return 1;
    }
    if p[1] != 1e1f32 {
        return 2;
    }
    if p[2] != 1e2f32 {
        return 3;
    }
    if p[3] != 1e3f32 {
        return 4;
    }
    if p[4] != 1e4f32 {
        return 5;
    }
    if p[5] != 1e5f32 {
        return 6;
    }
    if p[6] != 1e6f32 {
        return 7;
    }

    0
}