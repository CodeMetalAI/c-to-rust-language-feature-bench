fn main() {
    let n: usize = 4;
    let j: usize = 1;

    let mut saw_lab3 = false;
    let mut saw_lab4 = false;

    let mut out: i32 = 0;

    {
        let mut a = vec![0.0f64; n];

        enum State {
            Start,
            Lab3,
            Lab4,
        }

        let mut state = State::Start;

        loop {
            match state {
                State::Start => {
                    a[j] = 4.4;
                    out += 44;
                    state = State::Lab3;
                }
                State::Lab3 => {
                    saw_lab3 = true;
                    a[j] = 3.3;
                    out += 33;
                    state = State::Lab4;
                }
                State::Lab4 => {
                    saw_lab4 = true;
                    a[j] = 6.6;
                    out += 66;
                    break;
                }
            }
        }
    }

    if !saw_lab3 {
        std::process::exit(2);
    }
    if !saw_lab4 {
        std::process::exit(3);
    }
    if out != 143 {
        std::process::exit(4);
    }

    std::process::exit(0);
}