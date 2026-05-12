use std::process::exit;

fn main() {
    let n: usize = 4;
    let j: usize = 1;

    let mut saw_lab3: i32 = 0;
    let mut saw_lab4: i32 = 0;

    let mut out: i32 = 0;

    {
        let mut a = vec![0f64; n];

        a[j] = 4.4;
        out += 44;

        enum State {
            Lab3,
            Lab4,
            End,
        }

        let mut state = State::Lab3;

        loop {
            match state {
                State::Lab3 => {
                    saw_lab3 = 1;
                    a[j] = 3.3;
                    out += 33;
                    state = State::Lab4;
                }
                State::Lab4 => {
                    saw_lab4 = 1;
                    a[j] = 6.6;
                    out += 66;
                    state = State::End;
                }
                State::End => break,
            }
        }
    }

    if saw_lab3 == 0 {
        exit(2);
    }
    if saw_lab4 == 0 {
        exit(3);
    }
    if out != 143 {
        exit(4);
    }

    exit(0);
}