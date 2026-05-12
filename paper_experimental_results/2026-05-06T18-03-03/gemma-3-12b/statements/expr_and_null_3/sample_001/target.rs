fn main() {
    let mut x = 0;

    {
        goto end;
        x = 1;
    }

    end:
    x += 1;
    if x == 1 {
        0
    } else {
        1
    }
}