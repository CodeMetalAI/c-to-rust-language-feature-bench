**Test name**: rel_op_c6b

**Description:**
Validates C11 pointer equality cases where `==` is defined. The test checks that two null pointers
compare equal, that a pointer to an object compares equal to a pointer to a subobject at the beginning
of that object, and that two pointers one past the last element of the same array compare equal. The
non-portable case involving adjacent arrays in address space is permitted by the standard but is not
tested.
