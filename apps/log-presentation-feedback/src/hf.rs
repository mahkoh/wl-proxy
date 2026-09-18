use {
    crate::HfError,
    std::{fmt, mem, process::Command, rc::Rc},
    wl_proxy::{
        baseline::Baseline,
        object::{Object, ObjectCoreApi, ObjectRcUtils, ObjectUtils},
        protocols::{
            ObjectInterface,
            jay_wl_surface_factory_v1::{
                jay_wl_surface_factory_manager_v1::{
                    JayWlSurfaceFactoryManagerV1, JayWlSurfaceFactoryManagerV1Handler,
                },
                jay_wl_surface_factory_v1::{JayWlSurfaceFactoryV1, JayWlSurfaceFactoryV1Handler},
            },
            presentation_time::{
                wp_presentation::WpPresentation,
                wp_presentation_feedback::{
                    WpPresentationFeedback, WpPresentationFeedbackHandler,
                    WpPresentationFeedbackKind,
                },
            },
            wayland::{
                wl_callback::{WlCallback, WlCallbackHandler},
                wl_compositor::{WlCompositor, WlCompositorHandler},
                wl_display::{WlDisplay, WlDisplayHandler},
                wl_fixes::WlFixes,
                wl_registry::{WlRegistry, WlRegistryHandler},
                wl_surface::{WlSurface, WlSurfaceHandler},
            },
        },
        simple::{SimpleCommandExt, SimpleProxy},
    },
};

pub fn main(program: Vec<String>) -> Result<(), HfError> {
    let server = SimpleProxy::new(Baseline::ALL_OF_THEM).map_err(HfError::CreateServer)?;
    Command::new(&program[0])
        .args(&program[1..])
        .with_wayland_display(server.display())
        .spawn_and_forward_exit_code()
        .map_err(HfError::SpawnChild)?;
    let err = server.run(WlDisplayHandlerImpl::default);
    Err(HfError::ServerFailed(err))
}

#[derive(Default)]
struct WlDisplayHandlerImpl {
    init: bool,
    wp_presentation: Option<Rc<WpPresentation>>,
    wl_fixes: Option<Rc<WlFixes>>,
    registries_without_handlers: Vec<Rc<WlRegistry>>,
}

impl WlDisplayHandler for WlDisplayHandlerImpl {
    fn handle_get_registry(&mut self, slf: &Rc<WlDisplay>, registry: &Rc<WlRegistry>) {
        if !self.init {
            self.init = true;
            let wl_registry = slf.new_send_get_registry();
            wl_registry.set_handler(ProxyRegistry {
                display: slf.clone(),
            });
            let sync = slf.new_send_sync();
            sync.set_handler(FirstRoundtripHandler {
                wl_display: slf.clone(),
                wl_registry,
            });
        }
        if let Some(wp_presentation) = &self.wp_presentation {
            registry.set_handler(ClientGenericHandler {
                wp_presentation: wp_presentation.clone(),
            });
        } else {
            self.registries_without_handlers.push(registry.clone());
        }
        slf.send_get_registry(registry);
    }
}

struct ProxyRegistry {
    display: Rc<WlDisplay>,
}

impl WlRegistryHandler for ProxyRegistry {
    fn handle_global(
        &mut self,
        slf: &Rc<WlRegistry>,
        name: u32,
        interface: ObjectInterface,
        version: u32,
    ) {
        match interface {
            ObjectInterface::WpPresentation => {
                let obj = slf.state().create_object::<WpPresentation>(version.min(2));
                slf.send_bind(name, obj.clone());
                self.display
                    .get_handler_mut::<WlDisplayHandlerImpl>()
                    .wp_presentation = Some(obj);
            }
            ObjectInterface::WlFixes => {
                let obj = slf.state().create_object::<WlFixes>(1);
                slf.send_bind(name, obj.clone());
                self.display
                    .get_handler_mut::<WlDisplayHandlerImpl>()
                    .wl_fixes = Some(obj);
            }
            _ => {}
        }
    }
}

struct FirstRoundtripHandler {
    wl_display: Rc<WlDisplay>,
    wl_registry: Rc<WlRegistry>,
}

impl WlCallbackHandler for FirstRoundtripHandler {
    fn handle_done(&mut self, _slf: &Rc<WlCallback>, _callback_data: u32) {
        let display = &mut *self.wl_display.get_handler_mut::<WlDisplayHandlerImpl>();
        if let Some(wl_fixes) = display.wl_fixes.take() {
            wl_fixes.send_destroy_registry(&self.wl_registry);
            wl_fixes.send_destroy();
        }
        let Some(wp_presentation) = &display.wp_presentation else {
            eprintln!("Compositor does not support wp_presentation");
            std::process::exit(1);
        };
        for wl_registry in mem::take(&mut display.registries_without_handlers) {
            wl_registry.set_handler(ClientGenericHandler {
                wp_presentation: wp_presentation.clone(),
            });
        }
    }
}

#[derive(Clone)]
struct ClientGenericHandler {
    wp_presentation: Rc<WpPresentation>,
}

impl WlRegistryHandler for ClientGenericHandler {
    fn handle_bind(&mut self, slf: &Rc<WlRegistry>, name: u32, id: Rc<dyn Object>) {
        match id.interface() {
            ObjectInterface::WlCompositor => {
                id.downcast::<WlCompositor>().set_handler(self.clone());
            }
            ObjectInterface::JayWlSurfaceFactoryManagerV1 => {
                id.downcast::<JayWlSurfaceFactoryManagerV1>()
                    .set_handler(self.clone());
            }
            _ => {}
        }
        slf.send_bind(name, id);
    }
}

impl WlCompositorHandler for ClientGenericHandler {
    fn handle_create_surface(&mut self, slf: &Rc<WlCompositor>, id: &Rc<WlSurface>) {
        id.set_handler(self.clone());
        slf.send_create_surface(id);
    }
}

impl JayWlSurfaceFactoryManagerV1Handler for ClientGenericHandler {
    fn handle_create_factory(
        &mut self,
        slf: &Rc<JayWlSurfaceFactoryManagerV1>,
        id: &Rc<JayWlSurfaceFactoryV1>,
    ) {
        id.set_handler(self.clone());
        slf.send_create_factory(id);
    }
}

impl JayWlSurfaceFactoryV1Handler for ClientGenericHandler {
    fn handle_surface(&mut self, slf: &Rc<JayWlSurfaceFactoryV1>, id: &Rc<WlSurface>) {
        id.set_handler(self.clone());
        slf.send_surface(id);
    }
}

impl WlSurfaceHandler for ClientGenericHandler {
    fn handle_commit(&mut self, slf: &Rc<WlSurface>) {
        let wp_presentation_feedback = self.wp_presentation.new_send_feedback(slf);
        wp_presentation_feedback.set_handler(Feedback {
            surface_id: slf.client_id().unwrap(),
        });
        slf.send_commit();
    }
}

struct Feedback {
    surface_id: u32,
}

impl WpPresentationFeedbackHandler for Feedback {
    fn handle_presented(
        &mut self,
        _slf: &Rc<WpPresentationFeedback>,
        tv_sec_hi: u32,
        tv_sec_lo: u32,
        tv_nsec: u32,
        refresh: u32,
        seq_hi: u32,
        seq_lo: u32,
        flags: WpPresentationFeedbackKind,
    ) {
        let tv_sec = (tv_sec_hi as u64) << 32 | tv_sec_lo as u64;
        let tv_nsec = tv_nsec as u64;
        let nsec = tv_sec * 1_000_000_000 + tv_nsec;
        let seq = (seq_hi as u64) << 32 | seq_lo as u64;
        let flags = fmt::from_fn(|f| {
            if flags.contains(WpPresentationFeedbackKind::VSYNC) {
                f.write_str(",vsync")?;
            }
            if flags.contains(WpPresentationFeedbackKind::HW_CLOCK) {
                f.write_str(",hw_clock")?;
            }
            if flags.contains(WpPresentationFeedbackKind::HW_COMPLETION) {
                f.write_str(",hw_completion")?;
            }
            if flags.contains(WpPresentationFeedbackKind::ZERO_COPY) {
                f.write_str(",zero_copy")?;
            }
            Ok(())
        });
        let id = self.surface_id;
        eprintln!("s:{id},presented,t:{nsec},r:{refresh},q:{seq}{flags}");
    }

    fn handle_discarded(&mut self, _slf: &Rc<WpPresentationFeedback>) {
        eprintln!("s:{},discarded", self.surface_id);
    }
}
