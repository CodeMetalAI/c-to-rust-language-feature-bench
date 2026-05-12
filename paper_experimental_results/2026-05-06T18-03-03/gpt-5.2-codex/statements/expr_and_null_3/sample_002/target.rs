use std::process::exit;

fn main() {
    let mut x = 0;

    {
        'blk: {
            break 'blk;
            x = 1;
        }
    }

    x += 1;
    exit(if x == 1 { 0 } else { 1 });
}