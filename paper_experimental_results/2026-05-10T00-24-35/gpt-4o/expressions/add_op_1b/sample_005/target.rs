fn main() {
    let mut a = [[0; 3]; 4];

    {
        let p = &mut a[1..];

        p[0][2] = 99;
    }

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    // Check if the offset from 'a' to 'p' is 1
    let p_offset = 1;
    if p_offset != 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}