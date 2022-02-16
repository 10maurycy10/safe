# safe

## A collection of unsound rust functions, alowing you to:

- preform unchecked type casts ``std::mem::transmute`` style.

- create ``&'static T`` from ``&T``

- create ``&'static mut`` T from ``&mut T``

- Raw memory read (get data at address)

- Raw memory write (set data at address)

- Convert a ``&T`` into ``&'static mut T``

## All safe code!

look at ``src/lib.rs`` and the deps, no ``unsafe`` to be found!

This works by exploiting a long standing compiler bug: https://github.com/rust-lang/rust/issues/25860 aka ``fake-static`` bug
