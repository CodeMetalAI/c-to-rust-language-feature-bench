**Test name**: simple_asgn_3

**Description:**
C11 §6.5.16.1 (Simple assignment) Example 3.
Consider the fragment:

```
const char **cpp;
char *p;
const char c = 'A';
cpp = &p; // constraint violation
*cpp = &c; // valid
*p = 0; // valid
```

The first assignment is unsafe because it would allow the following valid code to attempt to change the
value of the const object c.
