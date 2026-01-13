use {
    crate::{
        WttError,
        wtt::{self, Color, Theme},
    },
    clap::{CommandFactory, Parser, ValueHint},
    clap_complete::Shell,
    std::{io::stdout, num::ParseIntError, sync::Arc},
    thiserror::Error,
};

/// This application turns any application that creates normal wayland windows into a tray
/// application.
#[derive(Parser, Debug)]
struct WindowToTray {
    /// Generate shell completions instead of running the program.
    #[clap(long, value_enum, value_name = "SHELL")]
    generate_completion: Option<Shell>,
    /// Set the color of the popup border.
    #[clap(
        long,
        value_parser = parse_color,
        default_value = "000000",
    )]
    border_color: Color,
    /// Set the width of the popup border.
    #[clap(long, default_value = "20")]
    border_width: u16,
    /// Set the icon theme used for icons using a symbolic name.
    #[clap(long, default_value = "hicolor")]
    icon_theme: String,
    /// Set the color used for recolorable SVG icons.
    #[clap(
        long,
        value_parser = parse_color,
        default_value = "c8c8c8",
    )]
    icon_color: Color,
    /// The program to execute.
    #[clap(
        trailing_var_arg = true,
        value_hint = ValueHint::CommandWithArguments,
        required_unless_present = "generate_completion",
    )]
    program: Option<Vec<String>>,
}

#[derive(Debug, Error)]
enum ParseColorError {
    #[error("argument length must be 1, 2, 3, 4, 6, or 8")]
    InvalidLength,
    #[error(transparent)]
    NotHex(ParseIntError),
}

fn parse_color(s: &str) -> Result<Color, ParseColorError> {
    let v = u32::from_str_radix(s, 16).map_err(ParseColorError::NotHex)?;
    let mut r;
    let mut g;
    let mut b;
    let mut a = 255;
    match s.len() {
        1 => {
            r = v as u8;
            r |= r << 4;
            g = r;
            b = r;
        }
        2 => {
            r = v as u8;
            g = r;
            b = r;
        }
        3 => {
            r = (v >> 8) as u8 & 0xf;
            r |= r << 4;
            g = (v >> 4) as u8 & 0xf;
            g |= g << 4;
            b = (v >> 0) as u8 & 0xf;
            b |= b << 4;
        }
        4 => {
            r = (v >> 12) as u8 & 0xf;
            r |= r << 4;
            g = (v >> 8) as u8 & 0xf;
            g |= g << 4;
            b = (v >> 4) as u8 & 0xf;
            b |= b << 4;
            a = (v >> 0) as u8 & 0xf;
            a |= a << 4;
        }
        6 => {
            r = (v >> 16) as u8;
            g = (v >> 8) as u8;
            b = (v >> 0) as u8;
        }
        8 => {
            r = (v >> 24) as u8;
            g = (v >> 16) as u8;
            b = (v >> 8) as u8;
            a = (v >> 0) as u8;
        }
        _ => return Err(ParseColorError::InvalidLength),
    }
    let a = a as f32 / 255.0;
    Ok(Color {
        r: (r as f32 / 255.0) * a,
        g: (g as f32 / 255.0) * a,
        b: (b as f32 / 255.0) * a,
        a,
    })
}

pub fn main() -> Result<(), WttError> {
    let args = WindowToTray::parse();
    if let Some(shell) = args.generate_completion {
        let stdout = stdout();
        let mut stdout = stdout.lock();
        clap_complete::generate(
            shell,
            &mut WindowToTray::command(),
            "window-to-tray",
            &mut stdout,
        );
        return Ok(());
    }
    let theme = Theme {
        icon_theme: Arc::new(args.icon_theme),
        icon_color: args.icon_color,
        border_color: args.border_color,
        border_width: args.border_width,
    };
    wtt::main(theme, &args.program.unwrap())
}
