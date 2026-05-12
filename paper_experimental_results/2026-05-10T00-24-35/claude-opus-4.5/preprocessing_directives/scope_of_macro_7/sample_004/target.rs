fn main() {
    let x = 7;
    let y = 3;

    // debug("Flag") counts 1 argument
    let debug_result_1 = 1;
    if debug_result_1 != 1 {
        std::process::exit(1);
    }

    // debug("X = %d\n", x) counts 2 arguments
    let debug_result_2 = 2;
    if debug_result_2 != 2 {
        std::process::exit(2);
    }

    // showlist(The first, second, and third items.) stringifies to "The first, second, and third items."
    let showlist_result = "The first, second, and third items.";
    if showlist_result != "The first, second, and third items." {
        std::process::exit(3);
    }

    // report(x > y, "x is %d but y is %d", x, y)
    // x > y is true (7 > 3), so returns 1
    let report_result_1 = if x > y { 1 } else { 3 }; // 3 arguments after test
    if report_result_1 != 1 {
        std::process::exit(4);
    }

    // report(x < y, "x is %d but y is %d", x, y)
    // x < y is false (7 < 3), so returns PP_NARG of 3 arguments = 3
    let report_result_2 = if x < y { 1 } else { 3 }; // 3 arguments after test
    if report_result_2 != 3 {
        std::process::exit(5);
    }

    std::process::exit(0);
}