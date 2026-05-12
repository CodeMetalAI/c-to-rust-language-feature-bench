fn main() {
    let mut expr = 0;

    switch_statement: {
        let mut i = 4;
        expr = 0;

        match expr {
            0 => {
                i = 17;
                if i == 17 {
                    return;
                } else {
                    return 1;
                }
            }
            _ => {}
        }
    }
}