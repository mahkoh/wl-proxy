use {
    crate::{CmError, cli::WlCmFilter},
    phf::phf_map,
    std::{collections::HashSet, process::Command, rc::Rc, sync::Arc},
    wl_proxy::{
        baseline::Baseline,
        object::{ConcreteObject, Object, ObjectCoreApi, ObjectRcUtils},
        protocols::{
            color_management_v1::wp_color_manager_v1::{
                WpColorManagerV1, WpColorManagerV1Feature, WpColorManagerV1Handler,
                WpColorManagerV1Primaries, WpColorManagerV1RenderIntent,
                WpColorManagerV1TransferFunction,
            },
            wayland::{
                wl_display::{WlDisplay, WlDisplayHandler},
                wl_registry::{WlRegistry, WlRegistryHandler},
            },
        },
        simple::{SimpleCommandExt, SimpleProxy},
    },
};

pub fn main(args: &WlCmFilter) -> Result<(), CmError> {
    let program = args.program.as_ref().unwrap();
    let server = SimpleProxy::new(Baseline::V2).map_err(CmError::CreateServer)?;
    Command::new(&program[0])
        .args(&program[1..])
        .with_wayland_display(server.display())
        .spawn_and_forward_exit_code()
        .map_err(CmError::SpawnChild)?;
    let filters = create_filters(args);
    let err = server.run(|| WlDisplayHandlerImpl {
        filters: filters.clone(),
    });
    Err(CmError::ServerFailed(err))
}

static TRANSFER_FUNCTIONS: phf::Map<&'static str, WpColorManagerV1TransferFunction> = phf_map! {
    "bt1886"             => WpColorManagerV1TransferFunction::BT1886,
    "gamma22"            => WpColorManagerV1TransferFunction::GAMMA22,
    "gamma28"            => WpColorManagerV1TransferFunction::GAMMA28,
    "st240"              => WpColorManagerV1TransferFunction::ST240,
    "ext_linear"         => WpColorManagerV1TransferFunction::EXT_LINEAR,
    "log_100"            => WpColorManagerV1TransferFunction::LOG_100,
    "log_316"            => WpColorManagerV1TransferFunction::LOG_316,
    "xvycc"              => WpColorManagerV1TransferFunction::XVYCC,
    "srgb"               => WpColorManagerV1TransferFunction::SRGB,
    "ext_srgb"           => WpColorManagerV1TransferFunction::EXT_SRGB,
    "st2084_pq"          => WpColorManagerV1TransferFunction::ST2084_PQ,
    "st428"              => WpColorManagerV1TransferFunction::ST428,
    "hlg"                => WpColorManagerV1TransferFunction::HLG,
    "compound_power_2_4" => WpColorManagerV1TransferFunction::COMPOUND_POWER_2_4,
};

static PRIMARIES: phf::Map<&'static str, WpColorManagerV1Primaries> = phf_map! {
    "srgb"         => WpColorManagerV1Primaries::SRGB,
    "pal_m"        => WpColorManagerV1Primaries::PAL_M,
    "pal"          => WpColorManagerV1Primaries::PAL,
    "ntsc"         => WpColorManagerV1Primaries::NTSC,
    "generic_film" => WpColorManagerV1Primaries::GENERIC_FILM,
    "bt2020"       => WpColorManagerV1Primaries::BT2020,
    "cie1931_xyz"  => WpColorManagerV1Primaries::CIE1931_XYZ,
    "dci_p3"       => WpColorManagerV1Primaries::DCI_P3,
    "display_p3"   => WpColorManagerV1Primaries::DISPLAY_P3,
    "adobe_rgb"    => WpColorManagerV1Primaries::ADOBE_RGB,
};

static FEATURES: phf::Map<&'static str, WpColorManagerV1Feature> = phf_map! {
    "icc_v2_v4"                       => WpColorManagerV1Feature::ICC_V2_V4,
    "parametric"                      => WpColorManagerV1Feature::PARAMETRIC,
    "set_primaries"                   => WpColorManagerV1Feature::SET_PRIMARIES,
    "set_tf_power"                    => WpColorManagerV1Feature::SET_TF_POWER,
    "set_luminances"                  => WpColorManagerV1Feature::SET_LUMINANCES,
    "set_mastering_display_primaries" => WpColorManagerV1Feature::SET_MASTERING_DISPLAY_PRIMARIES,
    "extended_target_volume"          => WpColorManagerV1Feature::EXTENDED_TARGET_VOLUME,
    "windows_scrgb"                   => WpColorManagerV1Feature::WINDOWS_SCRGB,
};

static RENDER_INTENTS: phf::Map<&'static str, WpColorManagerV1RenderIntent> = phf_map! {
    "perceptual"             => WpColorManagerV1RenderIntent::PERCEPTUAL,
    "relative"               => WpColorManagerV1RenderIntent::RELATIVE,
    "saturation"             => WpColorManagerV1RenderIntent::SATURATION,
    "absolute"               => WpColorManagerV1RenderIntent::ABSOLUTE,
    "relative_bpc"           => WpColorManagerV1RenderIntent::RELATIVE_BPC,
    "absolute_no_adaptation" => WpColorManagerV1RenderIntent::ABSOLUTE_NO_ADAPTATION,
};

#[derive(Default)]
struct Filters {
    render_intents: HashSet<WpColorManagerV1RenderIntent>,
    render_intents_default_allow: bool,
    features: HashSet<WpColorManagerV1Feature>,
    features_default_allow: bool,
    primaries: HashSet<WpColorManagerV1Primaries>,
    primaries_default_allow: bool,
    transfer_functions: HashSet<WpColorManagerV1TransferFunction>,
    transfer_functions_default_allow: bool,
}

fn create_filters(args: &WlCmFilter) -> Arc<Filters> {
    let mut filters = Filters {
        render_intents_default_allow: !args.invert || args.render_intents.is_none(),
        features_default_allow: !args.invert || args.features.is_none(),
        primaries_default_allow: !args.invert || args.primaries.is_none(),
        transfer_functions_default_allow: !args.invert || args.transfer_functions.is_none(),
        ..Default::default()
    };
    macro_rules! add {
        ($field:ident, $map:expr, $name:expr) => {
            if let Some(v) = &args.$field {
                for v in v {
                    let Some(v) = $map.get(v) else {
                        eprintln!("Unknown {} {}", $name, v);
                        continue;
                    };
                    filters.$field.insert(*v);
                }
            }
        };
    }
    add!(render_intents, RENDER_INTENTS, "render intent");
    add!(features, FEATURES, "feature");
    add!(primaries, PRIMARIES, "primary");
    add!(transfer_functions, TRANSFER_FUNCTIONS, "transfer function");
    Arc::new(filters)
}

struct WlDisplayHandlerImpl {
    filters: Arc<Filters>,
}

impl WlDisplayHandler for WlDisplayHandlerImpl {
    fn handle_get_registry(&mut self, slf: &Rc<WlDisplay>, registry: &Rc<WlRegistry>) {
        registry.set_handler(WlRegistryHandlerImpl {
            filters: self.filters.clone(),
        });
        slf.send_get_registry(registry);
    }
}

struct WlRegistryHandlerImpl {
    filters: Arc<Filters>,
}

impl WlRegistryHandler for WlRegistryHandlerImpl {
    fn handle_bind(&mut self, slf: &Rc<WlRegistry>, name: u32, id: Rc<dyn Object>) {
        if let WpColorManagerV1::INTERFACE = id.interface() {
            id.downcast::<WpColorManagerV1>()
                .set_handler(WpColorManagerV1HandlerImpl {
                    filters: self.filters.clone(),
                })
        }
        slf.send_bind(name, id);
    }
}

struct WpColorManagerV1HandlerImpl {
    filters: Arc<Filters>,
}

impl WpColorManagerV1Handler for WpColorManagerV1HandlerImpl {
    fn handle_supported_intent(
        &mut self,
        slf: &Rc<WpColorManagerV1>,
        render_intent: WpColorManagerV1RenderIntent,
    ) {
        let mut allow = self.filters.render_intents_default_allow;
        if self.filters.render_intents.contains(&render_intent) {
            allow = !allow;
        }
        if allow {
            slf.send_supported_intent(render_intent);
        }
    }

    fn handle_supported_feature(
        &mut self,
        slf: &Rc<WpColorManagerV1>,
        feature: WpColorManagerV1Feature,
    ) {
        let mut allow = self.filters.features_default_allow;
        if self.filters.features.contains(&feature) {
            allow = !allow;
        }
        if allow {
            slf.send_supported_feature(feature);
        }
    }

    fn handle_supported_tf_named(
        &mut self,
        slf: &Rc<WpColorManagerV1>,
        tf: WpColorManagerV1TransferFunction,
    ) {
        let mut allow = self.filters.transfer_functions_default_allow;
        if self.filters.transfer_functions.contains(&tf) {
            allow = !allow;
        }
        if allow {
            slf.send_supported_tf_named(tf);
        }
    }

    fn handle_supported_primaries_named(
        &mut self,
        slf: &Rc<WpColorManagerV1>,
        primaries: WpColorManagerV1Primaries,
    ) {
        let mut allow = self.filters.primaries_default_allow;
        if self.filters.primaries.contains(&primaries) {
            allow = !allow;
        }
        if allow {
            slf.send_supported_primaries_named(primaries);
        }
    }
}
