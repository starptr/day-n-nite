use super::SetError;
use super::ModuleError;
use super::Mode;
use dirs;
use serde::Deserialize;
use toml;
use std::fs;
use std::io::{self, Write};

#[derive(Deserialize)]
struct Colors {
    foreground: Option<String>,
    background: Option<String>,
    cursor_bg: Option<String>,
    _cursor_border: Option<String>,
    _cursor_fg: Option<String>,
    selection_bg: Option<String>,
    selection_fg: Option<String>,
    ansi: Option<[String; 8]>,
    brights: Option<[String; 8]>,
}

#[derive(Deserialize)]
struct Colorscheme {
    colors: Colors,
}

pub fn set(mode: Mode) -> Result<(), ModuleError> {
    let colors_directory = dirs::home_dir().unwrap().join(".config").join("wezterm").join("colors");
    let target_colorscheme = colors_directory.join(if mode == Mode::Day { "My Material.toml" } else { "My Bright Dark Alacritty.toml" });
    let colorscheme_toml = fs::read_to_string(target_colorscheme)
        .map_err(|_| ModuleError::Terminal(SetError::ReadFailure))?;

    let colorscheme: Colorscheme = toml::from_str(&colorscheme_toml)
        .map_err(|_| ModuleError::Terminal(SetError::ParseFailure))?;

    print_dynamic_colors(colorscheme.colors)
}

fn print_dynamic_colors(colors: Colors) -> Result<(), ModuleError> {
    if let Some(foreground) = colors.foreground {
        print_esc_sequences(10, foreground);
    }
    if let Some(background) = colors.background {
        print_esc_sequences(11, background);
    }
    if let Some(cursor_bg) = colors.cursor_bg {
        print_esc_sequences(12, cursor_bg);
    }
    if let Some(selection_bg) = colors.selection_bg {
        print_esc_sequences(17, selection_bg);
    }
    if let Some(selection_fg) = colors.selection_fg {
        print_esc_sequences(19, selection_fg);
    }
    if let Some(ansi) = colors.ansi {
        print_ansi(ansi, false);
    }
    if let Some(brights) = colors.brights {
        print_ansi(brights, true);
    }

    io::stdout().flush().unwrap();
    Ok(())
}

fn print_esc_sequences(field_code: i32, color: String) {
    print!("\x1b]{};{}\x07", field_code, color);
}

fn print_ansi(color_list: [String; 8], is_bright: bool) {
    print!("\x1b]4{}\x07",color_list.iter().enumerate() .fold(String::new(), |acc, (index, cur)| format!("{};{};{}", acc, if is_bright {index + 8} else {index}, cur)));
}
