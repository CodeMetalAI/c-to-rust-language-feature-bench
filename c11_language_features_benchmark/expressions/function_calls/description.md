**Test name**: function_calls

**Description:**
Validates the C11 function-call sequencing rule in §6.5.2.2 using the example shape `(*pf[f1()]) (f2(), f3() + f4())`.
The test permits any evaluation order for `f1`, `f2`, `f3`, and `f4`, but checks that all side effects from evaluating
the function designator (`pf[f1()]`) and the arguments (`f2()`, `f3() + f4()`) are completed before the target function
is entered.
