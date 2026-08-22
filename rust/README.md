# Kalt - Rust Backend

This is the backend Rust library for the Typst package `kalt`. Kalt evaluates
arbitrary nested Typst equations, so there is less need to rely on `calc`.

This library is standalone and intended for anyone who wants to use the advanced
type handling and parsing capabilities of `kalt` in their own projects.

For an implementation example, take a look at the Typst bindings for `kalt`
(available in git).

## Tests

The Typst visual test suite lives in [typst/test.typ](../typst/test.typ). It is
organized as a coverage manifest plus focused sections for scalars, vectors,
matrices, helper bindings, and rendered error surfaces.

When a backend operation is added, update the manifest and add a rendered
example in the matching section so the coverage stays obvious during review.
