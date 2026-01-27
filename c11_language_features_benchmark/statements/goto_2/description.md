**Test name**: goto_2

**Description:**
Validates C11 §6.8.6.1 (The goto statement) Example 2. The test declares a variable length array (VLA)
inside a block and uses `goto` to jump forward to labels that are within the same block scope. It checks
that the jumps are permitted within the VLA's scope and that statements between each `goto` and its
target label are skipped, as evidenced by the observed assigned values.
