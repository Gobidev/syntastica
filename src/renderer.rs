use std::borrow::Cow;

use crate::style::{Color, Style};

pub trait Renderer {
    fn head(&mut self) -> Cow<'static, str> {
        "".into()
    }

    fn newline(&mut self) -> Cow<'static, str> {
        "\n".into()
    }

    fn tail(&mut self) -> Cow<'static, str> {
        "".into()
    }

    fn unstyled<'a>(&mut self, text: &'a str) -> Cow<'a, str> {
        text.into()
    }

    fn styled<'a>(&mut self, text: &'a str, style: Style) -> Cow<'a, str>;
}

////////////////////////////////////////

pub struct HtmlRenderer;
impl Renderer for HtmlRenderer {
    fn newline(&mut self) -> Cow<'static, str> {
        "<br>".into()
    }

    fn unstyled(&mut self, text: &str) -> Cow<'static, str> {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace(' ', "&nbsp;<wbr>")
            .replace('\n', "<br>")
            .replace('\t', "&nbsp;&nbsp;&nbsp;&nbsp;<wbr>")
            .into()
    }

    fn styled(&mut self, text: &str, style: Style) -> Cow<'static, str> {
        let (r, g, b) = style.color().rgb();
        let mut css = format!("color: rgb({r}, {g}, {b});");
        if style.underline() && style.strikethrough() {
            css += "text-decoration: underline line-through;"
        } else if style.underline() {
            css += "text-decoration: underline;"
        } else if style.strikethrough() {
            css += "text-decoration: line-through;"
        }
        if style.bold() {
            css += "font-weight: bold;"
        }
        if style.italic() {
            css += "font-style: italic;"
        }
        format!("<span style=\"{css}\">{text}</span>").into()
    }
}

pub struct TerminalRenderer {
    background_color: Option<Color>,
}

impl TerminalRenderer {
    pub fn new(background_color: Option<Color>) -> Self {
        Self { background_color }
    }
}

impl Renderer for TerminalRenderer {
    fn head(&mut self) -> Cow<'static, str> {
        match self.background_color {
            Some(color) => {
                let (r, g, b) = color.rgb();
                format!("\x1b[48;2;{r};{g};{b}m").into()
            }
            None => "".into(),
        }
    }

    fn tail(&mut self) -> Cow<'static, str> {
        match self.background_color {
            Some(_) => "\x1b[0m".into(),
            None => "".into(),
        }
    }

    fn newline(&mut self) -> Cow<'static, str> {
        match self.background_color {
            Some(color) => {
                let (r, g, b) = color.rgb();
                format!("\x1b[0m\n\x1b[48;2;{r};{g};{b}m").into()
            }
            None => "\n".into(),
        }
    }

    fn styled(&mut self, text: &str, style: Style) -> Cow<'static, str> {
        let (r, g, b) = style.color().rgb();
        let mut params = format!("38;2;{r};{g};{b};");
        if style.underline() {
            params += "4;"
        }
        if style.strikethrough() {
            params += "9;"
        }
        if style.italic() {
            params += "3;"
        }
        if style.bold() {
            params += "1;"
        }
        // trim last `;`
        params.truncate(params.len() - 1);
        format!("\x1b[{params}m{text}\x1b[39;24;29;23;22m").into()
    }
}
