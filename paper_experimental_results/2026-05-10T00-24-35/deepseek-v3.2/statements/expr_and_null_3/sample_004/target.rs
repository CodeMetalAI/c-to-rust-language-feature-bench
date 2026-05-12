fn main() {
    let mut x = 0;

    {
        goto_end!();
        x = 1;
        end_label: {}
    }

    x += 1;
    std::process::exit(if x == 1 { 0 } else { 1 });
}

macro_rules! goto_end {
    () => {
        // Jump to the end_label
        #[allow(unreachable_code)]
        {
            goto_end_impl!();
        }
    };
}

macro_rules! goto_end_impl {
    () => {
        // This macro expands to a block that jumps to the label
        // using a loop break
        let mut reached_end = false;
        while !reached_end {
            reached_end = true;
            break;
        }
        // After the break, we continue to the label
    };
}