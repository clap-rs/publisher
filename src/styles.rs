use clap::builder::{styling::AnsiColor, Styles};

pub fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::BrightYellow.on_default())
        .usage(AnsiColor::BrightYellow.on_default())
        .literal(AnsiColor::BrightGreen.on_default())
        .placeholder(AnsiColor::BrightCyan.on_default())
}
