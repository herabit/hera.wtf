# `hera.wtf`

The source code for my site.

## Goals

* Exclusively for my use, this is not being developed with anyone besides myself in mind.
* Minimal (if any) javascript.
* Use [tree-sitter](https://tree-sitter.github.io/) for syntax highlighting.
* Use [djot](https://djot.net/) as the markup language. This *avoids* much (though not all) of
  the issues we have with markdown.
* Use [typst](https://typst.app/) for generating graphics, such as those for math. Should be trivial as much of typst's
  functionality is exposed as Rust libraries.
* Use [Sass](https://sass-lang.com) (and therefore the [grass](https://github.com/connorkees/grass) compiler) for CSS preprocessing.
* Use [lightningcss](https://lightningcss.dev/) for vendor prefixing, and possibly minification of the CSS.
* Possibly compile posts to PDFs, utilizing typst... This is not a priority, but would be nice to have.
* Figure out a reasonable reason why the blogging tool is named `glados` and not something more, well, clever. Our current excuse is
  ***Segmentation fault***.

## On Bevy

We figured that utilizing a game engine, or rather the guts of a game engine, would be an interesting approach to
solving the design problems within the Static Site Generator space. We'll see where this goes, so far, it looks
promising.
