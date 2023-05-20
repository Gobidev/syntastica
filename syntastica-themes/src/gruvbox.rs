//! The gruvbox themes in this module were extracted from <https://github.com/ellisonleao/gruvbox.nvim>

use std::collections::BTreeMap;

use syntastica::{
    config::{Config, ThemeValue},
    theme,
};

pub fn dark() -> Config {
    let mut palette = theme! {
        "bg0": "#282828",
        "bg1": "#3c3836",
        "bg2": "#504945",
        "bg3": "#665c54",
        "bg4": "#7c6f64",
        "fg0": "#fbf1c7",
        "fg1": "#ebdbb2",
        "fg2": "#d5c4a1",
        "fg3": "#bdae93",
        "fg4": "#a89984",
        "red": "#fb4934",
        "green": "#b8bb26",
        "yellow": "#fabd2f",
        "blue": "#83a598",
        "purple": "#d3869b",
        "aqua": "#8ec07c",
        "orange": "#fe8019",
        "neutral_red": "#cc241d",
        "neutral_green": "#98971a",
        "neutral_yellow": "#d79921",
        "neutral_blue": "#458588",
        "neutral_purple": "#b16286",
        "neutral_aqua": "#689d6a",
        "gray": "#928374",
    }
    .into_inner();
    palette.append(&mut theme());
    palette.into()
}

pub fn light() -> Config {
    let mut palette = theme! {
        "bg0": "#fbf1c7",
        "bg1": "#ebdbb2",
        "bg2": "#d5c4a1",
        "bg3": "#bdae93",
        "bg4": "#a89984",
        "fg0": "#282828",
        "fg1": "#3c3836",
        "fg2": "#504945",
        "fg3": "#665c54",
        "fg4": "#7c6f64",
        "red": "#9d0006",
        "green": "#79740e",
        "yellow": "#b57614",
        "blue": "#076678",
        "purple": "#8f3f71",
        "aqua": "#427b58",
        "orange": "#af3a03",
        "neutral_red": "#cc241d",
        "neutral_green": "#98971a",
        "neutral_yellow": "#d79921",
        "neutral_blue": "#458588",
        "neutral_purple": "#b16286",
        "neutral_aqua": "#689d6a",
        "gray": "#928374",
    }
    .into_inner();
    palette.append(&mut theme());
    palette.into()
}

fn theme() -> BTreeMap<String, ThemeValue> {
    theme! {
        "comment": {
            color: None,
            underline: false,
            strikethrough: false,
            italic: true,
            bold: false,
            link: "gray",
        },
        "preproc": "$aqua",
        "define": "$aqua",
        "operator": {
            color: None,
            underline: false,
            strikethrough: false,
            italic: false,
            bold: false,
            link: "orange",
        },
        "string": {
            color: None,
            underline: false,
            strikethrough: false,
            italic: true,
            bold: false,
            link: "green",
        },
        "string.regex": {
            color: None,
            underline: false,
            strikethrough: false,
            italic: true,
            bold: false,
            link: "green",
        },
        "character": "$purple",
        "boolean": "$purple",
        "number": "$purple",
        "float": "$purple",
        "function": {
            color: None,
            underline: false,
            strikethrough: false,
            italic: false,
            bold: true,
            link: "green",
        },
        "function.builtin": "$orange",
        "function.call": {
            color: None,
            underline: false,
            strikethrough: false,
            italic: false,
            bold: true,
            link: "green",
        },
        "function.macro": "$aqua",
        "method": {
            color: None,
            underline: false,
            strikethrough: false,
            italic: false,
            bold: true,
            link: "green",
        },
        "method.call": {
            color: None,
            underline: false,
            strikethrough: false,
            italic: false,
            bold: true,
            link: "green",
        },
        "constructor": "$orange",
        "parameter": "$blue",
        "keyword": "$red",
        "keyword.function": "$red",
        "keyword.operator": "$red",
        "keyword.return": "$red",
        "conditional": "$red",
        "repeat": "$red",
        "label": "$red",
        "include": "$aqua",
        "exception": "$red",
        "type": "$yellow",
        "type.builtin": "$yellow",
        "type.definition": "$yellow",
        "type.qualifier": "$yellow",
        "storageclass": "$orange",
        "attribute": "$aqua",
        "field": "$blue",
        "property": "$blue",
        "variable": "$fg1",
        "variable.builtin": "$orange",
        "constant": "$purple",
        "constant.builtin": "$orange",
        "constant.macro": "$aqua",
        "namespace": "$fg1",
        "symbol": "$blue",
        "text": "$fg1",
        "text.strong": {
            color: None,
            underline: false,
            strikethrough: false,
            italic: false,
            bold: true,
            link: "fg1",
        },
        "text.emphasis": {
            color: None,
            underline: false,
            strikethrough: false,
            italic: true,
            bold: false,
            link: "fg1",
        },
        "text.underline": {
            color: None,
            underline: true,
            strikethrough: false,
            italic: false,
            bold: false,
            link: "fg1",
        },
        "text.strike": {
            color: None,
            underline: false,
            strikethrough: true,
            italic: false,
            bold: false,
            link: "fg1",
        },
        "text.title": {
            color: None,
            underline: false,
            strikethrough: false,
            italic: false,
            bold: true,
            link: "green",
        },
        "text.literal": {
            color: None,
            underline: false,
            strikethrough: false,
            italic: true,
            bold: false,
            link: "green",
        },
        "text.uri": {
            color: None,
            underline: true,
            strikethrough: false,
            italic: false,
            bold: false,
            link: "blue",
        },
        "text.math": "$orange",
        "text.environment": "$aqua",
        "text.environment.name": "$yellow",
        "text.reference": "$purple",
        "text.todo": {
            color: None,
            underline: false,
            strikethrough: false,
            italic: true,
            bold: true,
            link: "bg0",
        },
        "text.note.comment": {
            color: None,
            underline: false,
            strikethrough: false,
            italic: false,
            bold: true,
            link: "purple",
        },
        "text.warning": {
            color: None,
            underline: false,
            strikethrough: false,
            italic: false,
            bold: true,
            link: "red",
        },
        "text.danger": {
            color: None,
            underline: false,
            strikethrough: false,
            italic: false,
            bold: true,
            link: "bg0",
        },
        "text.danger.comment": {
            color: None,
            underline: false,
            strikethrough: false,
            italic: false,
            bold: true,
            link: "fg0",
        },
        "text.diff.add": "$green",
        "text.diff.delete": "$red",
        "tag.attribute": "$blue",
        "macro": "$aqua",
        "structure": "$aqua",
    }
    .into_inner()
}
