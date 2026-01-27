**Test name**: logical_OR_op_c4

**Description:**
Validates C11 §6.5.14 logical OR (`||`) semantics. The test checks that operands are evaluated
left to right, that the second operand is not evaluated when the first compares unequal to
zero (short-circuiting), and that when the second operand is evaluated, all side effects of
the first operand are complete and visible to the second operand, demonstrating the
sequence point between their evaluations.
