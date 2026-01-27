**Test name**: stmt_switch_1

**Description:**
Positive test for §6.8.4.2 "The `switch` statement" (Example 1). Demonstrates that `case` labels do not introduce a new scope and that control may enter a `switch` statement at a `case` label, bypassing preceding statements (but not the declaration). The variable `i` is declared within the `switch` block and assigned only after control transfers to `case 0`, avoiding undefined behavior while validating correct control-flow and scoping semantics.
