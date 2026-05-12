fn main() {
    let mut expr = 0;

    switch_statement: {
        let mut i = 4;
        match expr {
            0 => {
                i = 17;
                return if i == 17 { 0 } else { 1 };
            }
            _ => {}
        }
    }
}