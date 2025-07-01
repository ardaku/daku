# Primitives

Daku defines a few shared types that can be used in the command definitions.
Occasionally, a command should use a custom type that packs data better, but it
should match similarly to another type defined here.

## Primitives
 - `val` Arbitrary untyped 32 bits
 - `int` 32-bit integer
 - `num` 32-bit floating point (May require all 1 bits for NaN representation)

---

 - `long` 64-bit integer
 - `half` 16-bit integer
 - `byte` 8-bit integer
 - `nybl` 4-bit integer

---

 - `ptr[T]` 32-bit address
 - `opt[T]` 32-bit address or Null (0)
