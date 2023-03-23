# nen Documentation

This documentation contains every function, core type, etc that comes with __nen__.

Non-[stdlib](std) elements are defined per interpreter (of the bytecode), whereas stdlib elements are defined in their respective `nen` files (e.g. std.io is defined in the file [std/io.nen](/std/io.nen).

The majority of non-stdlib functions are very low-level stack manipulation functions, and utility functions. Most of these begin with a `_` for easy identification. It is recommended not to use these in your programs unless you know what you're doing.

## Packages

[std](std) The standard library
