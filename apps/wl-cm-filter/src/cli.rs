use {
    crate::{CmError, cm},
    clap::{CommandFactory, Parser, ValueHint},
    clap_complete::Shell,
    std::io::stdout,
};

/// This application can be used to limit capabilities reported by wp-color-management-v1
#[derive(Parser, Debug)]
#[clap(verbatim_doc_comment)]
pub struct WlCmFilter {
    /// Generate shell completions instead of running the program.
    #[clap(long, value_enum, value_name = "SHELL")]
    generate_completion: Option<Shell>,
    /// Invert the selection.
    ///
    /// If this flag is used, only those values that are listed in a filter are passed
    /// through.
    #[clap(short, long)]
    pub invert: bool,
    /// The intents to filter.
    #[clap(
        long,
        value_hint = ValueHint::Other,
        value_delimiter = ',',
    )]
    pub render_intents: Option<Vec<String>>,
    /// The features to filter.
    #[clap(
        long,
        value_hint = ValueHint::Other,
        value_delimiter = ',',
    )]
    pub features: Option<Vec<String>>,
    /// The primaries to filter.
    #[clap(
        long,
        value_hint = ValueHint::Other,
        value_delimiter = ',',
    )]
    pub primaries: Option<Vec<String>>,
    /// The transfer functions to filter.
    #[clap(
        long,
        value_hint = ValueHint::Other,
        value_delimiter = ',',
    )]
    pub transfer_functions: Option<Vec<String>>,
    #[clap(
        trailing_var_arg = true,
        value_hint = ValueHint::CommandWithArguments,
        required_unless_present = "generate_completion",
    )]
    /// The program to run.
    pub program: Option<Vec<String>>,
}

pub fn main() -> Result<(), CmError> {
    let args = WlCmFilter::parse();
    if let Some(shell) = args.generate_completion {
        let stdout = stdout();
        let mut stdout = stdout.lock();
        clap_complete::generate(
            shell,
            &mut WlCmFilter::command(),
            "wl-cm-filter",
            &mut stdout,
        );
        return Ok(());
    }
    cm::main(&args)
}
