**Test name**: add_op_c8

**Description:**
Clause 8 coverage for Section 6.5.6
Pointer-integer addition and subtraction yield pointers within the same array (or one past the end), preserve the pointer type, allow one-past-the-end formation but forbid dereference there, and produce defined results only when staying within the array object.
