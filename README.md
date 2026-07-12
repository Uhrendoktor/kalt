# Kalt

Kalt evaluates arbitrary nested Typst equations and returns the result as Typst
content. It is meant for cases where `calc` gets awkward: deeply nested
expressions, complex numbers, matrices, vectors, and indexing.

Since the result is still Typst content, you can keep formatting, reusing, and
combining it in your document instead of ending up with a dead numeric result.

The repository is split into a Rust backend, Wasm bindings, and the Typst-side
package docs. If you want the user-facing examples, start with
[typst/README.md](typst/README.md).

## What It Covers

- scalars and complex numbers
- matrices and vectors
- indexing and slicing
- built-in operators and functions

## Layout

- [rust](rust) contains the `kalt` crate, which does the actual evaluation.
- [rust_bindings](rust_bindings) builds the Wasm wrapper used by Typst.
- [typst](typst) has the package README, examples, and assets.

## Build

From the repo root:

```bash
cargo build --workspace
```

To build the bindings only:

```bash
cargo build --release --target wasm32-unknown-unknown -p kalt_bindings
```

## Docs

- [Rust README](rust/README.md)
- [Typst README](typst/README.md)
