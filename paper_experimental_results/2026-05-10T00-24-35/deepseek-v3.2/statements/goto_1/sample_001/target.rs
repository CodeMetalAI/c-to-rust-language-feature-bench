fn main() {
    let mut acc = 0;
    let mut first_time_visited = false;

    for i in 0..5 {
        if !first_time_visited {
            first_time_visited = true;
            goto_first_time(&mut acc);
            continue;
        }

        acc += 1;
        if acc < 15 {
            acc = -15;
        }
        goto_first_time(&mut acc);
    }

    std::process::exit(if acc == 20 { 0 } else { 1 });
}

fn goto_first_time(acc: &mut i32) {
    *acc = 15;
}