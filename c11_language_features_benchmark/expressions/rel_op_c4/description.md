**Test name**: rel_op_c4

**Description:**
Validates C11 §6.5.8 pointer comparison rules for non-array objects. The test treats a pointer to a
single object as if it were a pointer to the first element of an array of length one, checking that
adding one yields a valid one-past-the-end pointer and that relational comparisons involving that
pointer behave accordingly.
