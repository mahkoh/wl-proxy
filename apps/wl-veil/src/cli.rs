use {
    crate::{VeilError, veil},
    clap::{CommandFactory, Parser, ValueHint},
    clap_complete::Shell,
    std::{collections::HashMap, io::stdout, num::ParseIntError, str::FromStr},
    thiserror::Error,
};

/// This application can be used to hide wayland globals from applications or to
/// downgrade their versions.
#[derive(Parser, Debug)]
struct WlVeil {
    /// Generate shell completions instead of running the program.
    #[clap(long, value_enum, value_name = "SHELL")]
    generate_completion: Option<Shell>,
    /// Invert the selection.
    ///
    /// If this flag is used, only those globals that are listed in a filter are passed
    /// through.
    #[clap(short, long, value_name = "SHELL")]
    invert: bool,
    #[clap(
        short,
        value_hint = ValueHint::Other,
        value_parser = parse_filters,
    )]
    /// The filters to apply.
    ///
    /// Each filter should either be `<global_name>` to filter the global outright or
    /// `<global_name>=<version>` to downgrade the global to that version.
    filter: Vec<HashMap<String, Option<u32>>>,
    #[clap(
        trailing_var_arg = true,
        value_hint = ValueHint::CommandWithArguments,
        required_unless_present = "generate_completion",
    )]
    /// The program to run.
    program: Option<Vec<String>>,
}

#[derive(Debug, Error)]
#[error("Could not parse version of {0}")]
struct ParseFilterError(String, #[source] ParseIntError);

fn parse_filters(s: &str) -> Result<HashMap<String, Option<u32>>, ParseFilterError> {
    let mut config = HashMap::new();
    for c in s.split(',') {
        let c = c.trim();
        if c.is_empty() {
            continue;
        }
        let (interface, version) = parse_filter(c)?;
        config.insert(interface, version);
    }
    Ok(config)
}

fn parse_filter(s: &str) -> Result<(String, Option<u32>), ParseFilterError> {
    let Some((interface, version)) = s.split_once('=') else {
        return Ok((s.to_string(), None));
    };
    let interface = interface.trim().to_string();
    match u32::from_str(version.trim()) {
        Ok(0) => Ok((interface, None)),
        Ok(v) => Ok((interface, Some(v))),
        Err(e) => Err(ParseFilterError(interface, e)),
    }
}

pub fn main() -> Result<(), VeilError> {
    let args = WlVeil::parse();
    if let Some(shell) = args.generate_completion {
        let stdout = stdout();
        let mut stdout = stdout.lock();
        clap_complete::generate(shell, &mut WlVeil::command(), "wl-veil", &mut stdout);
        return Ok(());
    }
    let mut filter = HashMap::new();
    for f in args.filter {
        filter.extend(f);
    }
    veil::main(args.invert, filter, args.program.unwrap())
}
