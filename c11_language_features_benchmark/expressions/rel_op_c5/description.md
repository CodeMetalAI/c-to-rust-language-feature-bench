**Test name**: rel_op_c5

**Description:**
Validates C11 §6.5.8 defined pointer comparisons. The test checks that pointers to the same object and
pointers one past the last element of the same array compare equal, that pointers to array elements with
larger subscripts compare greater, that a one-past-the-end pointer compares greater than pointers into
the same array, that pointers to later-declared structure members compare greater than pointers to
earlier members, and that pointers to members of the same union object compare equal. The test performs
only comparisons for which the standard defines behavior.
