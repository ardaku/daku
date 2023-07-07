# Types

Daku defines a few shared types that can be used in the command definitions.
Occasionally, a command should use a custom type that packs data better, but it
should match similarly to another type defined here.

## Primitives
 - `val` An arbitrary untyped 32 bits
 - `int` A 32-bit integer
 - `num` A 32-bit floating point (May require all 1 bits for NaN representation)

---

 - `half` A 16-bit integer
 - `byte` An 8-bit integer
 - `nybl` A 4-bit integer

---

 - `ptr[T]` A 32-bit address
 - `opt[T]` A 32-bit address or Null (0)
