use {
    crate::{VeilError, veil},
    clap::{CommandFactory, Parser, ValueHint},
    clap_complete::Shell,
    std::{collections::HashMap, io::stdout, num::ParseIntError, str::FromStr},
    thiserror::Error,
};

#[derive(Parser, Debug)]
struct WlVeil {
    #[clap(long, value_enum)]
    generate_completion: Option<Shell>,
    #[clap(
        short,
        value_hint = ValueHint::Other,
        value_parser = parse_filters,
    )]
    filter: Vec<HashMap<String, Option<u32>>>,
    #[clap(
        trailing_var_arg = true,
        value_hint = ValueHint::CommandWithArguments,
        required_unless_present = "generate_completion",
    )]
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
    veil::main(filter, args.program.unwrap())
}
