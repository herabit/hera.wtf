use tree_sitter_language::LanguageFn;

mod hack;
use hack::receive_lemons;

macro_rules! opt {
    () => { None };
    ($($tt:tt)+) => { Some($($tt)+) };
}

macro_rules! lang {
    (
        $(#[$attr:meta])*
        pub enum Lang {
            $(
                $(#[$lang_attr:meta])*
                $lang:ident {
                    name: $($name:tt)|+,
                    func: $func:expr
                    $(, highlights: $highlights:expr)?
                    $(, locals: $locals:expr)?
                    $(, injections: $injections:expr)?
                    $(, tags: $tags:expr)?
                    $(,)?
                }
            ),+
            $(,)?
        }
    ) => {
        $(#[$attr])*
        pub enum Lang {
            $(
                $(#[$lang_attr])*
                $lang,
            )+
        }

        impl Lang {
            /// A slice containing all supported [`Lang`]s.
            pub const ALL: &[Lang] = &[
                $(Lang::$lang,)+
            ];

            /// Returns the names of this language.
            #[inline]
            #[must_use]
            #[track_caller]
            pub const fn names(self) -> &'static [&'static str] {
                match self {
                    $(Lang::$lang => const { &[$($name,)+] },)+
                }
            }

            /// Create a new [`Lang`] from its name.
            #[must_use]
            #[track_caller]
            // #[unsafe(no_mangle)]
            pub fn from_name(name: &str) -> Option<Lang> {
                match name {
                    $($($name)|+ => Some(Lang::$lang),)+
                    _ => None,
                }
            }

            /// Returns a [`LanguageFn`] corresponding to this language.
            #[inline]
            #[must_use]
            #[track_caller]
            pub const fn func(self) -> LanguageFn {
                match self {
                    $(Lang::$lang => const { $func },)+
                }
            }

            /// Returns the highlights query for this language, if any.
            #[inline]
            #[must_use]
            #[track_caller]
            pub const fn highlights(self) -> Option<&'static str> {
                match self {
                    $(Lang::$lang => const { opt!($($highlights)?) },)+
                }
            }

            /// Returns the locals query for this language, if any.
            #[inline]
            #[must_use]
            #[track_caller]
            pub const fn locals(self) -> Option<&'static str> {
                match self {
                    $(Lang::$lang => const { opt!($($locals)?) },)+
                }
            }

            /// Returns the injections query for this language, if any.
            #[inline]
            #[must_use]
            #[track_caller]
            pub const fn injections(self) -> Option<&'static str> {
                match self {
                    $(Lang::$lang => const { opt!($($injections)?) },)+
                }
            }

            /// Returns the tags query for this language, if any.
            #[inline]
            #[must_use]
            #[track_caller]
            pub const fn tags(self) -> Option<&'static str> {
                match self {
                    $(Lang::$lang => const { opt!($($tags)?) },)+
                }
            }
        }
    };
}

// TODO: Add more languages, we just, here, well, frankly don't care to add all of the languages quite yet.
//
// Additionally we need to find high-quality queries, and to, well, credit them wherever possible.
lang! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[non_exhaustive]
   pub enum Lang {
       /// Tree sitter grammar for assembly.
       Asm {
           name: "asm" | "nasm",
           func: ::tree_sitter_asm::LANGUAGE,
           highlights: ::tree_sitter_asm::HIGHLIGHTS_QUERY,
       },

       /// Tree sitter grammar for bash.
       Bash {
           name: "bash",
           func: ::tree_sitter_bash::LANGUAGE,
           highlights: ::tree_sitter_bash::HIGHLIGHT_QUERY,
       },

       /// Tree sitter grammar for C.
       C {
           name: "c",
           func: ::tree_sitter_c::LANGUAGE,
           highlights: ::tree_sitter_c::HIGHLIGHT_QUERY,
           tags: ::tree_sitter_c::TAGS_QUERY,
       },

       /// Tree sitter grammar for C#.
       Csharp {
           name: "c-sharp" | "c_sharp" | "csharp" | "c#",
           func: ::tree_sitter_c_sharp::LANGUAGE,
           // highlights: include_str!("./queries/c_sharp/highlights.scm"),
           // tags: include_str!("./queries/c_sharp/tags.scm"),
       },

       /// Tree sitter grammar for CMake.
       Cmake {
           name: "cmake" | "c_make" | "c-make",
           func: ::tree_sitter_cmake::LANGUAGE,
           // highlights: include_str!("./queries/cmake/highlights.scm"),
           // injections: include_str!("./queries/cmake/injections.scm"),
       },

       /// Tree sitter grammar for C++.
       Cpp {
           name: "cpp" | "c++",
           func: ::tree_sitter_cpp::LANGUAGE,
           highlights: ::tree_sitter_cpp::HIGHLIGHT_QUERY,
       },

       /// Tree sitter grammar for CSS.
       Css {
           name: "css",
           func: ::tree_sitter_css::LANGUAGE,
           highlights: ::tree_sitter_css::HIGHLIGHTS_QUERY,
       },

       /// Tree sitter grammar for Cuda.
       Cuda {
           name: "cuda",
           func: ::tree_sitter_cuda::LANGUAGE,
           highlights: ::tree_sitter_cuda::HIGHLIGHTS_QUERY,
       },

       /// Tree sitter grammar for Dart.
       Dart {
           name: "dart",
           // NOTE: This is a workaround for the fact that this isn't... exposed
           //       properly.
           func: unsafe { receive_lemons(::tree_sitter_dart::language) },
           // highlights: include_str!("./queries/dart/highlights.scm"),
           // tags: include_str!("./queries/dart/tags.scm"),
       },

       /// Tree sitter grammar for Elixir.
       Elixir {
           name: "elixir",
           func: ::tree_sitter_elixir::LANGUAGE,
           highlights: ::tree_sitter_elixir::HIGHLIGHTS_QUERY,
           injections: ::tree_sitter_elixir::INJECTIONS_QUERY,
       },

       /// Tree sitter grammar for Elm.
       Elm {
           name: "elm",
           func: ::tree_sitter_elm::LANGUAGE,
           highlights: ::tree_sitter_elm::HIGHLIGHTS_QUERY,
           injections: ::tree_sitter_elm::INJECTIONS_QUERY,
           tags: ::tree_sitter_elm::TAGS_QUERY,
       },

       /// Tree sitter grammar for Erlang.
       Erlang {
           name: "erlang",
           func: ::tree_sitter_erlang::LANGUAGE,
           // highlights: include_str!("./queries/erlang/highlights.scm"),
       },

       /// Tree sitter grammar for F#.
       Fsharp {
           name: "f-sharp" | "f_sharp" | "fsharp" | "f#",
           func: ::tree_sitter_fsharp::LANGUAGE_FSHARP,
           highlights: ::tree_sitter_fsharp::HIGHLIGHTS_QUERY,
           locals: ::tree_sitter_fsharp::LOCALS_QUERY,
           injections: ::tree_sitter_fsharp::INJECTIONS_QUERY,
           tags: ::tree_sitter_fsharp::TAGS_QUERY,
       },

       /// Tree sitter grammar for F# Signatures.
       FsharpSignature {
           // This is likely the only one that could be used.
           name: "fsharp_signature",
           func: ::tree_sitter_fsharp::LANGUAGE_SIGNATURE,
       },

       /// Tree sitter grammar for GDScript.
       GdScript {
           name: "gdscript",
           func: ::tree_sitter_gdscript::LANGUAGE,
           // highlights: ::tree_sitter_gdscript::HI
       },

       /// Tree sitter grammar for Gleam.
       Gleam {
           name: "gleam",
           func: ::tree_sitter_gleam::LANGUAGE,
           highlights: ::tree_sitter_gleam::HIGHLIGHT_QUERY,
           locals: ::tree_sitter_gleam::LOCALS_QUERY,
           tags: ::tree_sitter_gleam::TAGS_QUERY,
       },

       /// Tree sitter grammar for GLSL.
       Glsl {
           name: "glsl",
           func: ::tree_sitter_glsl::LANGUAGE_GLSL,
           highlights: ::tree_sitter_glsl::HIGHLIGHTS_QUERY,
       },

       /// Tree sitter grammar for Go.
       Go {
           name: "go" | "golang",
           func: ::tree_sitter_go::LANGUAGE,
           highlights: ::tree_sitter_go::HIGHLIGHTS_QUERY,
           tags: ::tree_sitter_go::TAGS_QUERY,
       },

       /// Tree sitter grammar for Haskell.
       Haskell {
           name: "haskell",
           func: ::tree_sitter_haskell::LANGUAGE,
           highlights: ::tree_sitter_haskell::HIGHLIGHTS_QUERY,
           locals: ::tree_sitter_haskell::LOCALS_QUERY,
           injections: ::tree_sitter_haskell::INJECTIONS_QUERY,
       },

       /// Tree sitter grammar for HLSL.
       Hlsl {
           name: "hlsl",
           func: ::tree_sitter_hlsl::LANGUAGE_HLSL,
           // TODO: Add highlights... One day.
       },

       /// Tree sitter grammar for HTML.
       Html {
           name: "html",
           func: ::tree_sitter_html::LANGUAGE,
           highlights: ::tree_sitter_html::HIGHLIGHTS_QUERY,
           injections: ::tree_sitter_html::INJECTIONS_QUERY,
       },

       /// Tree sitter grammar for Java.
       Java {
           name: "java",
           func: ::tree_sitter_java::LANGUAGE,
           highlights: ::tree_sitter_java::HIGHLIGHTS_QUERY,
           tags: ::tree_sitter_java::TAGS_QUERY,
       },

       /// Tree sitter grammar for JavaScript.
       JavaScript {
           name: "javascript" | "java-script" | "java_script" | "js",
           func: ::tree_sitter_javascript::LANGUAGE,
           highlights: ::tree_sitter_javascript::HIGHLIGHT_QUERY,
           locals: ::tree_sitter_javascript::LOCALS_QUERY,
           injections: ::tree_sitter_javascript::INJECTIONS_QUERY,
           tags: ::tree_sitter_javascript::TAGS_QUERY,
       },

       /// Tree sitter grammar for JSX.
       Jsx {
           name: "jsx",
           func: ::tree_sitter_javascript::LANGUAGE,
           highlights: ::tree_sitter_javascript::JSX_HIGHLIGHT_QUERY,
           locals: ::tree_sitter_javascript::LOCALS_QUERY,
           injections: ::tree_sitter_javascript::INJECTIONS_QUERY,
           tags: ::tree_sitter_javascript::TAGS_QUERY,
       },

       /// Tree sitter grammar for JSDoc.
       JsDoc {
           name: "jsdoc",
           func: ::tree_sitter_jsdoc::LANGUAGE,
           highlights: ::tree_sitter_jsdoc::HIGHLIGHTS_QUERY,
       },

       /// Tree sitter grammar for JSON.
       Json {
           name: "json",
           func: ::tree_sitter_json::LANGUAGE,
           highlights: ::tree_sitter_json::HIGHLIGHTS_QUERY,
       },

       /// Tree sitter grammar for Julia.
       Julia {
           name: "julia",
           func: ::tree_sitter_julia::LANGUAGE,
           // highlights: include_str!("./queries/julia/highlights.scm"),
           // locals: include_str!("./queries/julia/locals.scm"),
       },

       /// Tree sitter grammar for Lua.
       Lua {
           name: "lua",
           func: ::tree_sitter_lua::LANGUAGE,
           highlights: ::tree_sitter_lua::HIGHLIGHTS_QUERY,
           locals: ::tree_sitter_lua::LOCALS_QUERY,
           injections: ::tree_sitter_lua::INJECTIONS_QUERY,
           tags: ::tree_sitter_lua::TAGS_QUERY,
       },

       /// Tree sitter grammar for Make.
       Make {
           name: "make",
           func: ::tree_sitter_make::LANGUAGE,
           highlights: ::tree_sitter_make::HIGHLIGHTS_QUERY,
       },
   }
}

impl Lang {
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn into_language(self) -> ::tree_sitter::Language {
        ::tree_sitter::Language::new(self.func())
    }
}

impl From<Lang> for ::tree_sitter::Language {
    fn from(lang: Lang) -> Self {
        lang.into_language()
    }
}
