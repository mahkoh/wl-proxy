use {
    crate::{
        PaperError,
        paper::{self, Config},
    },
    clap::{CommandFactory, Parser, ValueEnum, ValueHint},
    clap_complete::Shell,
    std::io::stdout,
    wl_proxy::protocols::wlr_layer_shell_unstable_v1::{
        zwlr_layer_shell_v1::ZwlrLayerShellV1Layer,
        zwlr_layer_surface_v1::ZwlrLayerSurfaceV1KeyboardInteractivity,
    },
};

/// This application can be used to run any regular wayland application as a
/// layer-shell application.
#[derive(Parser, Debug)]
struct WlPaper {
    /// Generate shell completions instead of running the program.
    #[clap(long, value_enum, value_name = "SHELL")]
    generate_completion: Option<Shell>,
    /// The top margin.
    #[clap(long, default_value = "0")]
    margin_top: i32,
    /// The right margin.
    #[clap(long, default_value = "0")]
    margin_right: i32,
    /// The bottom margin.
    #[clap(long, default_value = "0")]
    margin_bottom: i32,
    /// The left margin.
    #[clap(long, default_value = "0")]
    margin_left: i32,
    /// The keyboard interactivity.
    #[clap(long, default_value = "on-demand")]
    keyboard_interactivity: KeyboardInteractivity,
    /// The layer.
    #[clap(long, default_value = "background")]
    layer: Layer,
    /// The program to run.
    #[clap(
        trailing_var_arg = true,
        value_hint = ValueHint::CommandWithArguments,
        required_unless_present = "generate_completion",
    )]
    program: Option<Vec<String>>,
}

#[derive(ValueEnum, Debug, Copy, Clone, Hash, PartialEq)]
pub enum KeyboardInteractivity {
    None,
    Exclusive,
    OnDemand,
}

#[derive(ValueEnum, Debug, Copy, Clone, Hash, PartialEq)]
pub enum Layer {
    Background,
    Bottom,
    Top,
    Overlay,
}

pub fn main() -> Result<(), PaperError> {
    let args = WlPaper::parse();
    if let Some(shell) = args.generate_completion {
        let stdout = stdout();
        let mut stdout = stdout.lock();
        clap_complete::generate(shell, &mut WlPaper::command(), "wl-paper", &mut stdout);
        return Ok(());
    }
    let config = Config {
        keyboard_interactivity: match args.keyboard_interactivity {
            KeyboardInteractivity::None => ZwlrLayerSurfaceV1KeyboardInteractivity::NONE,
            KeyboardInteractivity::Exclusive => ZwlrLayerSurfaceV1KeyboardInteractivity::EXCLUSIVE,
            KeyboardInteractivity::OnDemand => ZwlrLayerSurfaceV1KeyboardInteractivity::ON_DEMAND,
        },
        layer: match args.layer {
            Layer::Background => ZwlrLayerShellV1Layer::BACKGROUND,
            Layer::Bottom => ZwlrLayerShellV1Layer::BOTTOM,
            Layer::Top => ZwlrLayerShellV1Layer::TOP,
            Layer::Overlay => ZwlrLayerShellV1Layer::OVERLAY,
        },
        margin_top: args.margin_top,
        margin_right: args.margin_right,
        margin_bottom: args.margin_bottom,
        margin_left: args.margin_left,
    };
    paper::main(config, &args.program.unwrap())
}
