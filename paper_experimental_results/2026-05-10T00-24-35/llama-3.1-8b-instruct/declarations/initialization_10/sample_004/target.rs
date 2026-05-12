// initialization_10.rs
struct Div {
    quot: i32,
    rem: i32,
}

fn main() {
    let answer = Div {
        quot: 2,
        rem: -1,
    };

    if answer.quot!= 2 {
        println!("test 1 failed");
        std::process::exit(1);
    }
    if answer.rem!= -1 {
        println!("test 2 failed");
        std::process::exit(2);
    }

    {
        let swapped = Div {
            rem: -3,
            quot: 4,
        };
        if swapped.quot!= 4 {
            println!("test 3 failed");
            std::process::exit(3);
        }
        if swapped.rem!= -3 {
            println!("test 4 failed");
            std::process::exit(4);
        }
    }

    println!("All tests passed");
    std::process::exit(0);
}