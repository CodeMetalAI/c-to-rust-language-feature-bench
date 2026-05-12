fn main() -> i32 {
    let arr = [1e0f32, 1e1f32, 1e2f32, 1e3f32, 1e4f32, 1e5f32, 1e6f32];

    if arr[0] != 1e0f32 { return 1; }
    if arr[1] != 1e1f32 { return 2; }
    if arr[2] != 1e2f32 { return 3; }
    if arr[3] != 1e3f32 { return 4; }
    if arr[4] != 1e4f32 { return 5; }
    if arr[5] != 1e5f32 { return 6; }
    if arr[6] != 1e6f32 { return 7; }

    0
}