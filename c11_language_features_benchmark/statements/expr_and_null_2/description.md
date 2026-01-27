**Test name**: expr_and_null_2

**Description:**
EXAMPLE 2 from Section 6.8.3 "Expression and Null Statements" in the C11 standard. A null statement can be used to
supply an empty loop body to the iteration statement. All necessary updates (increments) are are handled within the
loop's declaration itself. Specifically, the side effect of s++ (advancing the pointer) is sequenced before the next
loop test and before the next iteration's body.
