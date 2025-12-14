use {
    crate::{
        WttError,
        wtt::{self},
    },
    clap::{CommandFactory, Parser, ValueHint},
    clap_complete::Shell,
    std::io::stdout,
};

#[derive(Parser, Debug)]
struct WindowToTray {
    #[clap(long, value_enum)]
    generate_completion: Option<Shell>,
    #[clap(
        trailing_var_arg = true,
        value_hint = ValueHint::CommandWithArguments,
        required_unless_present = "generate_completion",
    )]
    program: Option<Vec<String>>,
}

pub fn main() -> Result<(), WttError> {
    let args = WindowToTray::parse();
    if let Some(shell) = args.generate_completion {
        let stdout = stdout();
        let mut stdout = stdout.lock();
        clap_complete::generate(shell, &mut WindowToTray::command(), "wl-veil", &mut stdout);
        return Ok(());
    }
    wtt::main(&args.program.unwrap())
}
