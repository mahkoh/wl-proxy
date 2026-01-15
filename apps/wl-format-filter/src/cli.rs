use {
    crate::{
        HfError,
        hf::{self, INVALID_MODIFIER, LINEAR_MODIFIER},
    },
    clap::{CommandFactory, Parser, ValueHint},
    clap_complete::Shell,
    phf::phf_map,
    std::io::stdout,
    thiserror::Error,
};

/// This application can be used to hide buffer formats from applications.
///
/// Filters can be specified with the --allow and --deny options.
///
/// For each format/modifier pair, the behavior is as follows:
///
/// - If it matches an allow rule, it is passed through.
/// - Otherwise, if it matches a deny rule, it is filtered out.
/// - Otherwise, it is passed through.
///
/// wl_shm formats implicitly use the linear modifier.
/// wl_drm and older zwp_linux_dmabuf_v1 versions implicitly use the invalid modifier.
///
/// Each filter should have one of the following formats:
///
/// - `<format>`
/// - `<format>:<modifier>`
///
/// `<format>` should have one of the following formats:
///
/// - `all` - to match all formats
/// - one of the format names from wayland.xml
/// - a four-character fourcc name
/// - a hexadecimal number prefixed with 0x
/// - a decimal number
///
/// If `<modifier>` is not specified, the filter applies to all modifiers.
/// Otherwise, `<modifier>` should have one of the following formats.
///
/// - `linear` - to match the linear modifier
/// - `invalid` - to match the invalid modifier
/// - a hexadecimal number prefixed with 0x
/// - a decimal number
#[derive(Parser, Debug)]
#[clap(verbatim_doc_comment)]
struct WlFormatFilter {
    /// Generate shell completions instead of running the program.
    #[clap(long, value_enum, value_name = "SHELL")]
    generate_completion: Option<Shell>,
    /// The formats to allow unconditionally.
    #[clap(
        short,
        long,
        value_hint = ValueHint::Other,
        value_parser = parse_filter,
        value_delimiter = ',',
        value_name = "FILTER",
    )]
    allow: Vec<Filter>,
    /// The formats to deny unless they are explicitly allowed.
    #[clap(
        short,
        long,
        value_hint = ValueHint::Other,
        value_parser = parse_filter,
        value_delimiter = ',',
        value_name = "FILTER",
    )]
    deny: Vec<Filter>,
    #[clap(
        trailing_var_arg = true,
        value_hint = ValueHint::CommandWithArguments,
        required_unless_present = "generate_completion",
    )]
    /// The program to run.
    program: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct Filter {
    pub format: Option<u32>,
    pub modifier: Option<u64>,
}

#[derive(Debug, Error)]
enum ParseFilterError {
    #[error("could not parse the format {0}")]
    ParseFormat(String),
    #[error("could not parse the modifier {0}")]
    ParseModifier(String),
}

fn parse_filter(s: &str) -> Result<Filter, ParseFilterError> {
    let (format, modifier) = match s.split_once(":") {
        Some((format, modifier)) => (format, Some(modifier)),
        None => (s, None),
    };
    let format = if format == "all" {
        None
    } else if let Some(v) = WAYLAND_FORMATS.get(format) {
        Some(*v)
    } else if format.len() == 4 && format.is_ascii() {
        let mut v = 0;
        for &c in s.as_bytes().iter().rev() {
            v = v << 8 | c as u32;
        }
        Some(v)
    } else if let Some(format) = format.strip_prefix("0x")
        && let Ok(v) = u32::from_str_radix(format, 16)
    {
        Some(v)
    } else if let Ok(v) = u32::from_str_radix(format, 10) {
        Some(v)
    } else {
        return Err(ParseFilterError::ParseFormat(format.into()));
    };
    let modifier = match modifier {
        None => None,
        Some(m) => {
            let v = if m == "invalid" {
                INVALID_MODIFIER
            } else if m == "linear" {
                LINEAR_MODIFIER
            } else if let Some(m) = m.strip_prefix("0x")
                && let Ok(v) = u64::from_str_radix(m, 16)
            {
                v
            } else if let Ok(v) = u64::from_str_radix(m, 10) {
                v
            } else {
                return Err(ParseFilterError::ParseModifier(m.into()));
            };
            Some(v)
        }
    };
    Ok(Filter { format, modifier })
}

static WAYLAND_FORMATS: phf::Map<&'static str, u32> = phf_map! {
      "argb8888"             => 0,
      "xrgb8888"             => 1,
      "c8"                   => 0x20203843,
      "rgb332"               => 0x38424752,
      "bgr233"               => 0x38524742,
      "xrgb4444"             => 0x32315258,
      "xbgr4444"             => 0x32314258,
      "rgbx4444"             => 0x32315852,
      "bgrx4444"             => 0x32315842,
      "argb4444"             => 0x32315241,
      "abgr4444"             => 0x32314241,
      "rgba4444"             => 0x32314152,
      "bgra4444"             => 0x32314142,
      "xrgb1555"             => 0x35315258,
      "xbgr1555"             => 0x35314258,
      "rgbx5551"             => 0x35315852,
      "bgrx5551"             => 0x35315842,
      "argb1555"             => 0x35315241,
      "abgr1555"             => 0x35314241,
      "rgba5551"             => 0x35314152,
      "bgra5551"             => 0x35314142,
      "rgb565"               => 0x36314752,
      "bgr565"               => 0x36314742,
      "rgb888"               => 0x34324752,
      "bgr888"               => 0x34324742,
      "xbgr8888"             => 0x34324258,
      "rgbx8888"             => 0x34325852,
      "bgrx8888"             => 0x34325842,
      "abgr8888"             => 0x34324241,
      "rgba8888"             => 0x34324152,
      "bgra8888"             => 0x34324142,
      "xrgb2101010"          => 0x30335258,
      "xbgr2101010"          => 0x30334258,
      "rgbx1010102"          => 0x30335852,
      "bgrx1010102"          => 0x30335842,
      "argb2101010"          => 0x30335241,
      "abgr2101010"          => 0x30334241,
      "rgba1010102"          => 0x30334152,
      "bgra1010102"          => 0x30334142,
      "yuyv"                 => 0x56595559,
      "yvyu"                 => 0x55595659,
      "uyvy"                 => 0x59565955,
      "vyuy"                 => 0x59555956,
      "ayuv"                 => 0x56555941,
      "nv12"                 => 0x3231564e,
      "nv21"                 => 0x3132564e,
      "nv16"                 => 0x3631564e,
      "nv61"                 => 0x3136564e,
      "yuv410"               => 0x39565559,
      "yvu410"               => 0x39555659,
      "yuv411"               => 0x31315559,
      "yvu411"               => 0x31315659,
      "yuv420"               => 0x32315559,
      "yvu420"               => 0x32315659,
      "yuv422"               => 0x36315559,
      "yvu422"               => 0x36315659,
      "yuv444"               => 0x34325559,
      "yvu444"               => 0x34325659,
      "r8"                   => 0x20203852,
      "r16"                  => 0x20363152,
      "rg88"                 => 0x38384752,
      "gr88"                 => 0x38385247,
      "rg1616"               => 0x32334752,
      "gr1616"               => 0x32335247,
      "xrgb16161616f"        => 0x48345258,
      "xbgr16161616f"        => 0x48344258,
      "argb16161616f"        => 0x48345241,
      "abgr16161616f"        => 0x48344241,
      "xyuv8888"             => 0x56555958,
      "vuy888"               => 0x34325556,
      "vuy101010"            => 0x30335556,
      "y210"                 => 0x30313259,
      "y212"                 => 0x32313259,
      "y216"                 => 0x36313259,
      "y410"                 => 0x30313459,
      "y412"                 => 0x32313459,
      "y416"                 => 0x36313459,
      "xvyu2101010"          => 0x30335658,
      "xvyu12_16161616"      => 0x36335658,
      "xvyu16161616"         => 0x38345658,
      "y0l0"                 => 0x304c3059,
      "x0l0"                 => 0x304c3058,
      "y0l2"                 => 0x324c3059,
      "x0l2"                 => 0x324c3058,
      "yuv420_8bit"          => 0x38305559,
      "yuv420_10bit"         => 0x30315559,
      "xrgb8888_a8"          => 0x38415258,
      "xbgr8888_a8"          => 0x38414258,
      "rgbx8888_a8"          => 0x38415852,
      "bgrx8888_a8"          => 0x38415842,
      "rgb888_a8"            => 0x38413852,
      "bgr888_a8"            => 0x38413842,
      "rgb565_a8"            => 0x38413552,
      "bgr565_a8"            => 0x38413542,
      "nv24"                 => 0x3432564e,
      "nv42"                 => 0x3234564e,
      "p210"                 => 0x30313250,
      "p010"                 => 0x30313050,
      "p012"                 => 0x32313050,
      "p016"                 => 0x36313050,
      "axbxgxrx106106106106" => 0x30314241,
      "nv15"                 => 0x3531564e,
      "q410"                 => 0x30313451,
      "q401"                 => 0x31303451,
      "xrgb16161616"         => 0x38345258,
      "xbgr16161616"         => 0x38344258,
      "argb16161616"         => 0x38345241,
      "abgr16161616"         => 0x38344241,
      "c1"                   => 0x20203143,
      "c2"                   => 0x20203243,
      "c4"                   => 0x20203443,
      "d1"                   => 0x20203144,
      "d2"                   => 0x20203244,
      "d4"                   => 0x20203444,
      "d8"                   => 0x20203844,
      "r1"                   => 0x20203152,
      "r2"                   => 0x20203252,
      "r4"                   => 0x20203452,
      "r10"                  => 0x20303152,
      "r12"                  => 0x20323152,
      "avuy8888"             => 0x59555641,
      "xvuy8888"             => 0x59555658,
      "p030"                 => 0x30333050,
      "rgb161616"            => 0x38344752,
      "bgr161616"            => 0x38344742,
      "r16f"                 => 0x48202052,
      "gr1616f"              => 0x48205247,
      "bgr161616f"           => 0x48524742,
      "r32f"                 => 0x46202052,
      "gr3232f"              => 0x46205247,
      "bgr323232f"           => 0x46524742,
      "abgr32323232f"        => 0x46384241,
      "nv20"                 => 0x3032564e,
      "nv30"                 => 0x3033564e,
      "s010"                 => 0x30313053,
      "s210"                 => 0x30313253,
      "s410"                 => 0x30313453,
      "s012"                 => 0x32313053,
      "s212"                 => 0x32313253,
      "s412"                 => 0x32313453,
      "s016"                 => 0x36313053,
      "s216"                 => 0x36313253,
      "s416"                 => 0x36313453,
};

pub fn main() -> Result<(), HfError> {
    let args = WlFormatFilter::parse();
    if let Some(shell) = args.generate_completion {
        let stdout = stdout();
        let mut stdout = stdout.lock();
        clap_complete::generate(
            shell,
            &mut WlFormatFilter::command(),
            "wl-format-filter",
            &mut stdout,
        );
        return Ok(());
    }
    hf::main(args.allow, args.deny, args.program.unwrap())
}
