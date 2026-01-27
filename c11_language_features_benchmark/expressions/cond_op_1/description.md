**Test name**: cond_op_1

**Description:**
Validates C11 §6.5.15 (Conditional operator) Example 1. The test checks the common result type of
conditional expressions whose second and third operands are pointers by passing the result of each
conditional expression to a function that requires the expected pointer type. Each case from the
standard's example table is exercised (in both operand orders), ensuring that qualifier propagation
and type selection follow the specified rules and do not depend on whether the pointer operand types
are compatible.
