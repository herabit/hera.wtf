# `::glados_highlight`

What is this? Well, this is where, or rather the primitives we use for syntax highlighting.

We wrap [tree-sitter](https://tree-sitter.github.io/) and utilize that for the syntax highlighting, getting
"IDE-grade" syntax highlighting for our blog.

This is nice, but it has a few detriments, namely file size (we need to statically link multiple C libraries), as well as
compile time.

As such, we separated all of this logic from the rest of the static site generator, a crate is a *single* codgen unit, reducing
compile times for the rest of the generator.

Doubly, it allows the root `Cargo.toml` to not be, obscenely horrid.
