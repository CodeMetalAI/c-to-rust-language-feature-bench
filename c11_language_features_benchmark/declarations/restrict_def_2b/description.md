**Test name**: restrict_def_2b

**Description:**
Section 6.7.3.1 (restrict), Example 2b coverage.
Implements `f(n, int * restrict p, int * restrict q)` and a caller in the style of `g` that performs one non-overlapping call and one overlapping call. The program remains strictly defined by dynamically detecting overlap and routing overlapping cases to a memmove-like `safe_move`, while using `f` only when the restrict non-aliasing precondition holds. This stresses translation by making the UB-overlap call syntactically resemble a valid call site and by using an opaque (volatile-influenced) runtime overlap check whose correctness is validated against a shadow execution.
