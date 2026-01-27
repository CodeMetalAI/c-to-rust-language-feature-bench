**Test name**: rel_op_c7

**Description:**
Validates C11 §6.5.9 equality operator rules for pointers to non-array objects. The test treats a pointer
to a single object as if it were a pointer to the first element of an array of length one, checks that
one-past-the-end pointers derived from that object compare equal, and confirms that the object pointer
does not compare equal to its one-past-the-end pointer.
