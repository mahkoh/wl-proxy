use {
    crate::{HfError, hf},
    clap::{CommandFactory, Parser, ValueHint},
    clap_complete::Shell,
    std::io::stdout,
};

/// This application prints all dma-buf feedback sent to the application.
#[derive(Parser, Debug)]
#[clap(verbatim_doc_comment)]
struct WlFormatFilter {
    /// Generate shell completions instead of running the program.
    #[clap(long, value_enum, value_name = "SHELL")]
    generate_completion: Option<Shell>,
    #[clap(
        trailing_var_arg = true,
        value_hint = ValueHint::CommandWithArguments,
        required_unless_present = "generate_completion",
    )]
    /// The program to run.
    program: Option<Vec<String>>,
}

pub fn main() -> Result<(), HfError> {
    let args = WlFormatFilter::parse();
    if let Some(shell) = args.generate_completion {
        let stdout = stdout();
        let mut stdout = stdout.lock();
        clap_complete::generate(
            shell,
            &mut WlFormatFilter::command(),
            "log-dmabuf-feedback",
            &mut stdout,
        );
        return Ok(());
    }
    hf::main(args.program.unwrap())
}
