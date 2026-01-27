**Test name**: expr_and_null_3

**Description:**
EXAMPLE 3 from Section 6.8.3 "Expression and Null Statements" in the C11 standard.
A null statement may also be used to carry a label just before the closing `}` of a compound statement.
The key idea is that labels in C must label a statement (i.e., `end_loop1:` must label a statement),
and a null statement `;` is a valid statement that does nothing.
