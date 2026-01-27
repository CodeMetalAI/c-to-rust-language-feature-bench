**Test name**: comma_op_1

**Description:**
Example 1 in 6.5.17 "Comma operator". As indicated by the syntax, the comma operator (as described in this subclause)
cannot appear in contexts where a comma is used to separate items in a list (such as arguments to functions or lists
of initializers). On the other hand, it can be used within a parenthesized expression or within the second expression of
a conditional operator in such contexts. In the function call `f(a, (t=3, t+2), c)` the function has three arguments,
the second of which has the value `5`.
