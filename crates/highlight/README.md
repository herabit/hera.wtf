# `::glados_highlight`

What is this? Well, this is where, or rather the primitives we use for syntax highlighting, live.

We wrap [tree-sitter](https://tree-sitter.github.io/) and utilize that for the syntax highlighting, getting
"IDE-grade" syntax highlighting for our blog.

This is separated into its own crate for a few reasons:

1. Reducing compile times.
2. Preventing the workspace's `Cargo.toml` from getting too chaotic.
