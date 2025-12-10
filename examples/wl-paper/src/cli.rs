use {
    crate::{
        PaperError,
        paper::{self, Config},
    },
    clap::{CommandFactory, Parser, ValueHint},
    clap_complete::Shell,
    std::io::stdout,
};

#[derive(Parser, Debug)]
struct WlPaper {
    #[clap(long, value_enum)]
    generate_completion: Option<Shell>,
    #[clap(long)]
    margin_top: Option<i32>,
    #[clap(long)]
    margin_right: Option<i32>,
    #[clap(long)]
    margin_bottom: Option<i32>,
    #[clap(long)]
    margin_left: Option<i32>,
    #[clap(
        trailing_var_arg = true,
        value_hint = ValueHint::CommandWithArguments,
        required_unless_present = "generate_completion",
    )]
    program: Option<Vec<String>>,
}

pub fn main() -> Result<(), PaperError> {
    let args = WlPaper::parse();
    if let Some(shell) = args.generate_completion {
        let stdout = stdout();
        let mut stdout = stdout.lock();
        clap_complete::generate(shell, &mut WlPaper::command(), "wl-veil", &mut stdout);
        return Ok(());
    }
    let config = Config {
        margin_top: args.margin_top.unwrap_or(0),
        margin_right: args.margin_right.unwrap_or(0),
        margin_bottom: args.margin_bottom.unwrap_or(0),
        margin_left: args.margin_left.unwrap_or(0),
    };
    paper::main(config, &args.program.unwrap())
}
