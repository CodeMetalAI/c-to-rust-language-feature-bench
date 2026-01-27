**Test name**: size_of_3

**Description:**
Validates C11 §6.5.3.4 (The `sizeof` and` \_Alignof` operators) Example 3. The test checks that applying
`sizeof` to a variable length array yields a value computed at execution time. Specifically, it
verifies that `sizeof b` evaluates to `n + 3` when `b` is declared as `char b[n+3]`, and that
`fsize3(10)` returns 13.
