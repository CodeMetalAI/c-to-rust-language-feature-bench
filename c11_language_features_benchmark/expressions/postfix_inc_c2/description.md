**Test name**: postfix_inc_c2

**Description:**
Validates C11 §6.5.2.4 postfix increment semantics for an atomic integer object. The test checks that
`E++` evaluates to the value of `E` prior to the increment, and that as a side effect the stored value
of `E` is incremented by one. It also validates the required sequencing: the result value is computed
before the stored value is updated, even though the operation is a single atomic read-modify-write.
