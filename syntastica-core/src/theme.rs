//! Defines items related to theming the output.

use std::{borrow::Borrow, collections::BTreeMap, ops::Index};

use crate::{
    style::{Color, Style},
    Error, Result,
};

/// All theme keys that are recognized by syntastica.
///
/// A [`Theme`] or [`ResolvedTheme`] should define styles for any subset of these. View the source
/// to see the full, commented list.
///
/// The list is based on the list from
/// [nvim-treesitter](https://github.com/nvim-treesitter/nvim-treesitter/blob/master/CONTRIBUTING.md).
#[rustfmt::skip] // rustfmt messes up the comment alignment
pub const THEME_KEYS: &[&str] = &[
    "comment",               // line and block comments
    "comment.documentation", // comments documenting code
    "error",                 // syntax/parser errors
    "none",                  // completely disable the highlight
    "preproc",               // various preprocessor directives & shebangs
    "define",                // preprocessor definition directives
    "operator",              // symbolic operators (e.g. `+` / `*`)

    // punctuation
    "punctuation.delimiter", // delimiters (e.g. `;` / `.` / `,`)
    "punctuation.bracket",   // brackets (e.g. `()` / `{}` / `[]`)
    "punctuation.special",   // special symbols (e.g. `{}` in string interpolation)

    // literals
    "string",                // string literals
    "string.documentation",  // string documenting code (e.g. Python docstrings)
    "string.regex",          // regular expressions
    "string.escape",         // escape sequences
    "string.special",        // other special strings (e.g. dates)
    "character",             // character literals
    "character.special",     // special characters (e.g. wildcards)
    "boolean",               // boolean literals
    "number",                // numeric literals
    "float",                 // floating-point number literals

    // functions
    "function",              // function definitions
    "function.builtin",      // built-in functions
    "function.call",         // function calls
    "function.macro",        // preprocessor macros
    "method",                // method definitions
    "method.call",           // method calls
    "constructor",           // constructor calls and definitions
    "parameter",             // parameters of a function

    // keywords
    "keyword",               // various keywords
    "keyword.coroutine",     // keywords related to coroutines (e.g. `go` in Go, `async/await` in Python)
    "keyword.function",      // keywords that define a function (e.g. `func` in Go, `def` in Python)
    "keyword.operator",      // operators that are English words (e.g. `and` / `or`)
    "keyword.return",        // keywords like `return` and `yield`
    "conditional",           // keywords related to conditionals (e.g. `if` / `else`)
    "conditional.ternary",   // ternary operator (e.g. `?` / `:`)
    "repeat",                // keywords related to loops (e.g. `for` / `while`)
    "debug",                 // keywords related to debugging
    "label",                 // GOTO and other labels (e.g. `label:` in C)
    "include",               // keywords for including modules (e.g. `import` / `from` in Python)
    "exception",             // keywords related to exceptions (e.g. `throw` / `catch`)

    // types
    "type",                  // type or class definitions and annotations
    "type.builtin",          // built-in types
    "type.definition",       // type definitions (e.g. `typedef` in C)
    "type.qualifier",        // type qualifiers (e.g. `const`)
    "storageclass",          // modifiers that affect storage in memory or life-time
    "attribute",             // attribute annotations (e.g. Python decorators)
    "field",                 // object and struct fields
    "property",              // similar to `@field`

    // identifiers
    "variable",              // various variable names
    "variable.builtin",      // built-in variable names (e.g. `this`)
    "constant",              // constant identifiers
    "constant.builtin",      // built-in constant values
    "constant.macro",        // constants defined by the preprocessor
    "namespace",             // modules or namespaces
    "symbol",                // symbols or atoms

    // text
    "text",                  // non-structured text
    "text.strong",           // bold text
    "text.emphasis",         // text with emphasis
    "text.underline",        // underlined text
    "text.strike",           // strikethrough text
    "text.title",            // text that is part of a title
    "text.literal",          // literal or verbatim text (e.g., inline code)
    "text.quote",            // text quotations
    "text.uri",              // URIs (e.g. hyperlinks)
    "text.math",             // math environments (e.g. `$ ... $` in LaTeX)
    "text.environment",      // text environments of markup languages
    "text.environment.name", // text indicating the type of an environment
    "text.reference",        // text references, footnotes, citations, etc.
    "text.todo",             // todo notes
    "text.note",             // info notes
    "text.warning",          // warning notes
    "text.danger",           // danger/error notes
    "text.diff.add",         // added text (for diff files)
    "text.diff.delete",      // deleted text (for diff files)

    // tags
    "tag",                   // XML tag names
    "tag.attribute",         // XML tag attributes
    "tag.delimiter",         // XML tag delimiters
];

/// A raw theme which may contain links to other items inside.
///
/// Internally, this type stores a map from [`String`]s to [`ThemeValue`]s. This map can be
/// retrieved using [`Theme::into_inner`]. The map keys do _not_ all have to be in [`THEME_KEYS`];
/// other custom keys can be used, for example to define a set of colors and reuse them with links
/// everywhere else.
///
/// When using the <span class="stab portability"><code>serde</code></span> feature, this type
/// implements serde's `Serialize` and `Deserialize` traits.
///
/// # Instantiation
///
/// The easiest way to create a [`Theme`] is with the [`theme!`](crate::theme!) macro.
/// Alternatively, a [`Theme`] may be created from a [`BTreeMap<String, ThemeValue>`] using
/// [`Theme::new`].
#[derive(Clone, Hash, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Theme(BTreeMap<String, ThemeValue>);

/// A [`Theme`] where all internal links have been resolved.
///
/// Instead of [`ThemeValue`]s, a [`ResolvedTheme`] has [`Style`]s as values. These cannot link to
/// other entries of the theme but completely define a style on their own.
///
/// A [`ResolvedTheme`] can be created from a [`Theme`] with [`Theme::resolve_links`] or the
/// [`TryFrom<Theme>`](#impl-TryFrom<Theme>-for-ResolvedTheme) implementation.
#[derive(Clone, Hash, Debug, PartialEq, Eq)]
pub struct ResolvedTheme(BTreeMap<String, Style>);

/// A value of a [`Theme`] containing style information and/or a link to another key in the
/// [`Theme`].
///
/// When using the <span class="stab portability"><code>serde</code></span> feature, this type
/// implements serde's `Serialize` and `Deserialize` traits using the untagged enum representation.
#[derive(Clone, Hash, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ThemeValue {
    /// May either be a hexadecimal color literal, or a `$` followed by the name of another
    /// theme key.
    ///
    /// In the latter case, this value links to the [`ThemeValue`] which the [`Theme`] specifies
    /// for the provided theme key.
    Simple(String),
    /// A color or link with additional style information.
    Extended {
        /// The color to use for this style.
        ///
        /// Either this or [`link`](ThemeValue::Extended::link) has to be set, or calls to
        /// [`Theme::resolve_links`] will fail.
        color: Option<String>,

        /// Whether the text should be underlined. (default is `false`)
        #[cfg_attr(feature = "serde", serde(default))]
        underline: bool,

        /// Whether the text should be strikethrough. (default is `false`)
        #[cfg_attr(feature = "serde", serde(default))]
        strikethrough: bool,

        /// Whether the text should be italic. (default is `false`)
        #[cfg_attr(feature = "serde", serde(default))]
        italic: bool,

        /// Whether the text should be bold. (default is `false`)
        #[cfg_attr(feature = "serde", serde(default))]
        bold: bool,

        /// A link to the theme entry with the given key.
        ///
        /// Either this or [`color`](ThemeValue::Extended::color) has to be set, or calls to
        /// [`Theme::resolve_links`] will fail.
        link: Option<String>,
    },
}

impl Theme {
    pub fn new(highlights: BTreeMap<String, ThemeValue>) -> Self {
        Self(highlights)
    }

    pub fn into_inner(self) -> BTreeMap<String, ThemeValue> {
        self.0
    }

    pub fn resolve_links(mut self) -> Result<ResolvedTheme> {
        self.resolve_links_impl()?;
        Ok(ResolvedTheme::new(
            self.0
                .into_iter()
                .map(|(key, value)| {
                    Ok((
                        key,
                        match value {
                            ThemeValue::Simple(color) => {
                                Style::new(Color::from_hex(color)?, false, false, false, false)
                            }
                            ThemeValue::Extended {
                                color,
                                underline,
                                strikethrough,
                                italic,
                                bold,
                                link: _,
                            } => Style::new(
                                // TODO: maybe rework to not rely on unwrapping
                                Color::from_hex(color.expect("links have been resolved"))?,
                                underline,
                                strikethrough,
                                italic,
                                bold,
                            ),
                        },
                    ))
                })
                .collect::<Result<_>>()?,
        ))
    }

    fn resolve_links_impl(&mut self) -> Result<()> {
        let mut must_reresolve = false;
        let mut replacements = vec![];
        for (key, value) in self.0.iter() {
            let link_key = match value {
                ThemeValue::Simple(str) if str.starts_with('$') => &str[1..],
                ThemeValue::Extended {
                    link: Some(str), ..
                } => str,
                _ => continue,
            };
            let resolved = value.resolve_link(
                self.0
                    .get(link_key)
                    .ok_or_else(|| Error::InvalidLink(link_key.to_owned()))?,
            );
            if matches!(&resolved, ThemeValue::Simple(str) if str.starts_with('$'))
                || matches!(&resolved, ThemeValue::Extended { link: Some(_), .. })
            {
                must_reresolve = true;
            }
            replacements.push((key.clone(), resolved));
        }
        for (key, replacement) in replacements {
            *self.0.get_mut(&key).expect("key validity checked above") = replacement;
        }
        if must_reresolve {
            self.resolve_links_impl()?;
        }
        Ok(())
    }
}

impl From<BTreeMap<String, ThemeValue>> for Theme {
    fn from(highlights: BTreeMap<String, ThemeValue>) -> Self {
        Self::new(highlights)
    }
}

impl ResolvedTheme {
    pub fn new(highlights: BTreeMap<String, Style>) -> Self {
        Self(highlights)
    }

    pub fn into_inner(self) -> BTreeMap<String, Style> {
        self.0
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&Style>
    where
        String: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.0.get(key)
    }
}

impl<Q> Index<&Q> for ResolvedTheme
where
    String: Borrow<Q>,
    Q: Ord + ?Sized,
{
    type Output = Style;

    fn index(&self, key: &Q) -> &Self::Output {
        self.get(key).expect("no entry found for key")
    }
}

impl From<BTreeMap<String, Style>> for ResolvedTheme {
    fn from(highlights: BTreeMap<String, Style>) -> Self {
        Self::new(highlights)
    }
}

impl TryFrom<Theme> for ResolvedTheme {
    type Error = Error;

    fn try_from(value: Theme) -> Result<Self> {
        value.resolve_links()
    }
}

impl ThemeValue {
    fn resolve_link(&self, target: &Self) -> Self {
        match (self, target) {
            (ThemeValue::Simple(_), _) => target.clone(),
            (
                ThemeValue::Extended {
                    color: Some(color),
                    underline,
                    strikethrough,
                    italic,
                    bold,
                    link: _,
                },
                ThemeValue::Simple(_),
            )
            | (
                ThemeValue::Extended {
                    color: None,
                    underline,
                    strikethrough,
                    italic,
                    bold,
                    link: _,
                },
                ThemeValue::Simple(color),
            ) => Self::Extended {
                color: Some(color.clone()),
                underline: *underline,
                strikethrough: *strikethrough,
                italic: *italic,
                bold: *bold,
                link: None,
            },
            (
                ThemeValue::Extended {
                    color: color @ Some(_),
                    underline,
                    strikethrough,
                    italic,
                    bold,
                    link: _,
                },
                ThemeValue::Extended {
                    color: _,
                    underline: other_underline,
                    strikethrough: other_strikethrough,
                    italic: other_italic,
                    bold: other_bold,
                    link,
                },
            )
            | (
                ThemeValue::Extended {
                    color: None,
                    underline,
                    strikethrough,
                    italic,
                    bold,
                    link: _,
                },
                ThemeValue::Extended {
                    color,
                    underline: other_underline,
                    strikethrough: other_strikethrough,
                    italic: other_italic,
                    bold: other_bold,
                    link,
                },
            ) => Self::Extended {
                color: color.clone(),
                underline: *underline || *other_underline,
                strikethrough: *strikethrough || *other_strikethrough,
                italic: *italic || *other_italic,
                bold: *bold || *other_bold,
                link: link.clone(),
            },
        }
    }
}

#[macro_export(local_inner_macros)]
macro_rules! theme {
    ($($tt:tt)*) => {
        theme_impl!($($tt)*)
    };
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! theme_impl {
    () => {};
    ($($key:literal : $value:tt),* $(,)?) => {{
        let mut theme = ::std::collections::BTreeMap::new();
        $(
            theme.insert($key.to_owned(), theme_impl!(@value $value));
        )*
        $crate::theme::Theme::new(theme)
    }};
    (@value $str:literal) => {
        $crate::theme::ThemeValue::Simple($str.to_owned())
    };
    (@value {
        color: $color:tt,
        underline: $underline:expr,
        strikethrough: $strikethrough:expr,
        italic: $italic:expr,
        bold: $bold:expr,
        link: $link:tt $(,)?
    }) => {
        $crate::theme::ThemeValue::Extended {
            color: theme_impl!(@option $color),
            underline: $underline,
            strikethrough: $strikethrough,
            italic: $italic,
            bold: $bold,
            link: theme_impl!(@option $link),
        }
    };
    (@option None) => { None };
    (@option $str:literal) => { Some($str.to_owned()) };
}
