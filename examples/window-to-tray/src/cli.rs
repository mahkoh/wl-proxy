use {
    crate::{
        WttError,
        icon::ThemeColor,
        wtt::{self},
    },
    clap::{CommandFactory, Parser, ValueHint},
    clap_complete::Shell,
    std::{io::stdout, num::ParseIntError},
    thiserror::Error,
};

#[derive(Parser, Debug)]
struct WindowToTray {
    #[clap(long, value_enum)]
    generate_completion: Option<Shell>,
    #[clap(
        long,
        value_hint = ValueHint::Other,
        value_parser = parse_color,
        default_value = "000000",
    )]
    border_color: ThemeColor,
    #[clap(long, default_value = "20")]
    border_width: u16,
    #[clap(
        long,
        value_hint = ValueHint::Other,
        value_parser = parse_color,
        default_value = "c8c8c8",
    )]
    icon_color: ThemeColor,
    #[clap(
        trailing_var_arg = true,
        value_hint = ValueHint::CommandWithArguments,
        required_unless_present = "generate_completion",
    )]
    program: Option<Vec<String>>,
}

#[derive(Debug, Error)]
enum ParseColorError {
    #[error("argument length must be 3, 4, 6, or 8")]
    InvalidLength,
    #[error(transparent)]
    NotHex(ParseIntError),
}

fn parse_color(s: &str) -> Result<ThemeColor, ParseColorError> {
    let v = u32::from_str_radix(s, 16).map_err(ParseColorError::NotHex)?;
    let mut r;
    let mut g;
    let mut b;
    let mut a = 255;
    match s.len() {
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
    Ok(ThemeColor {
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
        clap_complete::generate(shell, &mut WindowToTray::command(), "wl-veil", &mut stdout);
        return Ok(());
    }
    wtt::main(
        args.border_color,
        args.icon_color,
        args.border_width,
        &args.program.unwrap(),
    )
}
