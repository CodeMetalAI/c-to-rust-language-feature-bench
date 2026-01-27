**Test name**: simple_asgn_2

**Description:**
C11 §6.5.16.1 (Simple assignment) Example 2.
In the fragment:

```
char c;
int i;
long l;
l = (c = i);
```

the value of `i` is converted to the type of the assignment expression `c = i`, that is, `char` type. The value
of the expression enclosed in parentheses is then converted to the type of the outer assignment expression,
that is, `long int` type.
