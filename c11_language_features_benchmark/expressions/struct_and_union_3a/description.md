**Test name**: struct_and_union_3a

**Description:**
Validates C11 §6.5.2.3: if a union contains several structures that share a common initial sequence, and if the union
object currently contains one of these structures, it is permitted to inspect the common
initial part of any of them anywhere that a declaration of the completed type of the union
is visible. Two structures share a common initial sequence if corresponding members
have compatible types (and, for bit-fields, the same widths) for a sequence of one or more
initial members.
