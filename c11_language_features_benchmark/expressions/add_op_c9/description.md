**Test name**: add_op_c9

**Description:**
Validates C11 §6.5.6 pointer subtraction semantics. The test checks that subtracting two pointers into the
same array yields the difference of their element subscripts with type `ptrdiff_t`. It also checks the
relationships involving a pointer to one past the last element: with `Q` pointing to the last element,
`(Q+1) - P` equals `(Q - P) + 1` and equals `-(P - (Q+1))`, and `(Q+1) - (Q+1)` evaluates to zero.
