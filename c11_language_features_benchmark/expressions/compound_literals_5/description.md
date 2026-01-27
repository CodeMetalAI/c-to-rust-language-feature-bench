**Test name**: compound_literal_5

**Description:**
Validates C11 §6.5.2.5 (Compound literals) Example 5. The test distinguishes a string literal from two
compound-literal array forms. It creates (1) a pointer to a string literal (static storage duration),
(2) a `char[]` compound literal (automatic storage duration in a function body and modifiable), and
(3) a `const char[]` compound literal (automatic storage duration in a function body and read-only).
The test modifies the `char[]` compound literal and verifies the change.
