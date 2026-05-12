fn main() {
    let mut x = 0;

    {
        'blk: {
            break 'blk;
            x = 1;
        }
    }

    x += 1;
    if x == 1 {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}