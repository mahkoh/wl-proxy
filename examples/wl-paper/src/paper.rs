use {
    crate::PaperError,
    std::{
        any::Any,
        cell::{Cell, RefCell},
        process::Command,
        rc::Rc,
        sync::Arc,
    },
    wl_proxy::{
        protocols::{
            ProxyInterface,
            wayland::{
                wl_callback::{WlCallback, WlCallbackHandler},
                wl_display::{WlDisplay, WlDisplayHandler},
                wl_fixes::WlFixes,
                wl_output::WlOutput,
                wl_registry::{WlRegistry, WlRegistryHandler},
                wl_seat::WlSeat,
                wl_surface::WlSurface,
            },
            wlr_layer_shell_unstable_v1::{
                zwlr_layer_shell_v1::{ZwlrLayerShellV1, ZwlrLayerShellV1Layer},
                zwlr_layer_surface_v1::{
                    ZwlrLayerSurfaceV1, ZwlrLayerSurfaceV1Anchor, ZwlrLayerSurfaceV1Handler,
                    ZwlrLayerSurfaceV1KeyboardInteractivity,
                },
            },
            xdg_shell::{
                xdg_popup::{XdgPopup, XdgPopupHandler},
                xdg_positioner::{XdgPositioner, XdgPositionerHandler},
                xdg_surface::{XdgSurface, XdgSurfaceHandler},
                xdg_toplevel::{
                    XdgToplevel, XdgToplevelHandler, XdgToplevelResizeEdge, XdgToplevelState,
                },
                xdg_wm_base::{XdgWmBase, XdgWmBaseHandler},
            },
        },
        proxy::{Proxy, ProxyUtils},
        simple::{SimpleCommandExt, SimpleServer},
    },
};

pub fn main(config: Config, program: &[String]) -> Result<(), PaperError> {
    let config = Arc::new(config);
    let server = SimpleServer::new().map_err(PaperError::CreateServer)?;
    Command::new(&program[0])
        .args(&program[1..])
        .with_wayland_display(server.display())
        .spawn_and_forward_exit_code()
        .map_err(PaperError::SpawnChild)?;
    let err = server.run(|| DisplayHandler {
        init: false,
        layer_shell: Default::default(),
        config: config.clone(),
    });
    Err(PaperError::ServerFailed(err))
}

pub struct Config {
    pub margin_top: i32,
    pub margin_right: i32,
    pub margin_bottom: i32,
    pub margin_left: i32,
}

#[derive(Default)]
struct LayerShell {
    layer_shell: RefCell<Option<Rc<ZwlrLayerShellV1>>>,
}

struct DisplayHandler {
    init: bool,
    layer_shell: Rc<LayerShell>,
    config: Arc<Config>,
}

impl WlDisplayHandler for DisplayHandler {
    fn get_registry(&mut self, slf: &Rc<WlDisplay>, registry: &Rc<WlRegistry>) {
        if !self.init {
            self.init = true;
            let registry = slf.create_child::<WlRegistry>();
            registry.set_handler(FirstRegistryHandler {
                layer_shell: self.layer_shell.clone(),
                wl_fixes: Default::default(),
            });
            let _ = slf.send_get_registry(&registry);
            let sync = slf.create_child::<WlCallback>();
            sync.set_handler(FirstSyncHandler { registry });
            let _ = slf.send_sync(&sync);
        }
        let _ = slf.send_get_registry(registry);
        registry.set_handler(RegistryHandler {
            layer_shell: self.layer_shell.clone(),
            config: self.config.clone(),
        });
    }
}

struct FirstRegistryHandler {
    layer_shell: Rc<LayerShell>,
    wl_fixes: Option<Rc<WlFixes>>,
}

impl WlRegistryHandler for FirstRegistryHandler {
    fn global(&mut self, slf: &Rc<WlRegistry>, name: u32, interface: &str, version: u32) {
        match interface {
            ZwlrLayerShellV1::INTERFACE => {
                let proxy = slf.state().create_proxy::<ZwlrLayerShellV1>(version.min(5));
                let _ = slf.send_bind(name, proxy.clone());
                *self.layer_shell.layer_shell.borrow_mut() = Some(proxy);
            }
            WlFixes::INTERFACE => {
                let proxy = slf.state().create_proxy::<WlFixes>(1);
                let _ = slf.send_bind(name, proxy.clone());
                self.wl_fixes = Some(proxy);
            }
            _ => {}
        }
    }
}

struct FirstSyncHandler {
    registry: Rc<WlRegistry>,
}

impl WlCallbackHandler for FirstSyncHandler {
    fn done(&mut self, _slf: &Rc<WlCallback>, _callback_data: u32) {
        let rh = self
            .registry
            .get_handler_ref::<FirstRegistryHandler>()
            .unwrap();
        if let Some(fixes) = &rh.wl_fixes {
            let _ = fixes.send_destroy_registry(&self.registry);
            let _ = fixes.send_destroy();
        }
        if rh.layer_shell.layer_shell.borrow().is_none() {
            eprintln!(
                "compositor does not support {}",
                ZwlrLayerShellV1::INTERFACE,
            );
            std::process::exit(1);
        }
    }
}

struct RegistryHandler {
    layer_shell: Rc<LayerShell>,
    config: Arc<Config>,
}

impl WlRegistryHandler for RegistryHandler {
    fn bind(&mut self, slf: &Rc<WlRegistry>, name: u32, id: Rc<dyn Proxy>) {
        if id.interface() == ProxyInterface::XdgWmBase
            && let Some(layer_shell) = &*self.layer_shell.layer_shell.borrow()
        {
            let id = (id.clone() as Rc<dyn Any>).downcast::<XdgWmBase>().unwrap();
            id.set_handler(WmBaseHandler {
                layer_shell: layer_shell.clone(),
                config: self.config.clone(),
            });
        }
        let _ = slf.send_bind(name, id);
    }
}

struct WmBaseHandler {
    layer_shell: Rc<ZwlrLayerShellV1>,
    config: Arc<Config>,
}

impl XdgWmBaseHandler for WmBaseHandler {
    fn create_positioner(&mut self, slf: &Rc<XdgWmBase>, id: &Rc<XdgPositioner>) {
        id.set_handler(PositionerHandler {
            anchor_rect: Default::default(),
        });
        let _ = slf.send_create_positioner(id);
    }

    fn get_xdg_surface(
        &mut self,
        slf: &Rc<XdgWmBase>,
        id: &Rc<XdgSurface>,
        surface: &Rc<WlSurface>,
    ) {
        let ss = Rc::new(XdgSurfaceShared {
            layer_shell: self.layer_shell.clone(),
            surface: surface.clone(),
            xdg_surface: id.clone(),
            xdg_wm_base: slf.clone(),
            geometry: Default::default(),
        });
        id.set_handler(XdgSurfaceHandlerImpl {
            surface: ss.clone(),
            layer_surface: None,
            has_server: false,
            config: self.config.clone(),
        });
    }
}

struct XdgSurfaceShared {
    layer_shell: Rc<ZwlrLayerShellV1>,
    surface: Rc<WlSurface>,
    xdg_surface: Rc<XdgSurface>,
    xdg_wm_base: Rc<XdgWmBase>,
    geometry: Cell<Option<[i32; 4]>>,
}

struct XdgSurfaceHandlerImpl {
    surface: Rc<XdgSurfaceShared>,
    layer_surface: Option<Rc<ZwlrLayerSurfaceV1>>,
    has_server: bool,
    config: Arc<Config>,
}

struct ToplevelHandler {
    layer_surface: Rc<ZwlrLayerSurfaceV1>,
}

struct LayerSurfaceHandler {
    xdg: Rc<XdgSurfaceShared>,
    toplevel: Rc<XdgToplevel>,
}

impl XdgSurfaceHandler for XdgSurfaceHandlerImpl {
    fn destroy(&mut self, slf: &Rc<XdgSurface>) {
        slf.unset_handler();
        if self.has_server {
            let _ = slf.send_destroy();
            self.has_server = false;
        }
    }

    fn get_toplevel(&mut self, _slf: &Rc<XdgSurface>, id: &Rc<XdgToplevel>) {
        let layer_surface = self
            .surface
            .layer_shell
            .create_child::<ZwlrLayerSurfaceV1>();
        let _ = self.surface.layer_shell.send_get_layer_surface(
            &layer_surface,
            &self.surface.surface,
            None,
            ZwlrLayerShellV1Layer::BACKGROUND,
            "",
        );
        let c = &*self.config;
        let _ = layer_surface.send_set_size(0, 0);
        let _ = layer_surface.send_set_margin(
            c.margin_top,
            c.margin_right,
            c.margin_bottom,
            c.margin_left,
        );
        let _ = layer_surface
            .send_set_keyboard_interactivity(ZwlrLayerSurfaceV1KeyboardInteractivity::ON_DEMAND);
        let _ = layer_surface.send_set_anchor(
            ZwlrLayerSurfaceV1Anchor::TOP
                | ZwlrLayerSurfaceV1Anchor::BOTTOM
                | ZwlrLayerSurfaceV1Anchor::LEFT
                | ZwlrLayerSurfaceV1Anchor::RIGHT,
        );
        id.set_handler(ToplevelHandler {
            layer_surface: layer_surface.clone(),
        });
        layer_surface.set_handler(LayerSurfaceHandler {
            xdg: self.surface.clone(),
            toplevel: id.clone(),
        });
        self.layer_surface = Some(layer_surface.clone());
    }

    fn get_popup(
        &mut self,
        slf: &Rc<XdgSurface>,
        id: &Rc<XdgPopup>,
        parent: Option<&Rc<XdgSurface>>,
        positioner: &Rc<XdgPositioner>,
    ) {
        if !self.has_server {
            self.has_server = true;
            let _ = self
                .surface
                .xdg_wm_base
                .send_get_xdg_surface(slf, &self.surface.surface);
        }
        if let Some([x, y, w, h]) = self.surface.geometry.get() {
            let _ = slf.send_set_window_geometry(x, y, w, h);
        }
        let mut layer_parent = None;
        let mut direct_parent = parent;
        let mut parent_geometry = None;
        if let Some(parent) = parent
            && let Ok(handler) = parent.get_handler_ref::<XdgSurfaceHandlerImpl>()
            && let Some(parent) = &handler.layer_surface
        {
            layer_parent = Some(parent.clone());
            direct_parent = None;
            parent_geometry = handler.surface.geometry.get();
        }
        let pos_handler = &*positioner.get_handler_ref::<PositionerHandler>().unwrap();
        if let Some([gx, gy, _, _]) = parent_geometry
            && let Some([px, py, pw, ph]) = pos_handler.anchor_rect
        {
            let _ = positioner.send_set_anchor_rect(gx + px, gy + py, pw, ph);
        }
        let _ = slf.send_get_popup(id, direct_parent, positioner);
        if parent_geometry.is_some()
            && let Some([px, py, pw, ph]) = pos_handler.anchor_rect
        {
            let _ = positioner.send_set_anchor_rect(px, py, pw, ph);
        }
        if let Some(layer_parent) = layer_parent {
            let _ = layer_parent.send_get_popup(id);
        }
        if let Some(pg) = parent_geometry {
            id.set_handler(XdgPopupHandlerImpl {
                parent_geometry: pg,
            });
        }
    }

    fn set_window_geometry(
        &mut self,
        slf: &Rc<XdgSurface>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        self.surface.geometry.set(Some([x, y, width, height]));
        if self.has_server {
            let _ = slf.send_set_window_geometry(x, y, width, height);
        }
    }

    fn ack_configure(&mut self, slf: &Rc<XdgSurface>, serial: u32) {
        if let Some(tl) = &self.layer_surface {
            let _ = tl.send_ack_configure(serial);
        } else if self.has_server {
            let _ = slf.send_ack_configure(serial);
        }
    }
}

impl XdgToplevelHandler for ToplevelHandler {
    fn destroy(&mut self, slf: &Rc<XdgToplevel>) {
        slf.unset_handler();
        self.layer_surface.unset_handler();
        let _ = self.layer_surface.send_destroy();
    }

    fn set_parent(&mut self, _slf: &Rc<XdgToplevel>, _parent: Option<&Rc<XdgToplevel>>) {
        // nothing
    }

    fn set_title(&mut self, _slf: &Rc<XdgToplevel>, _title: &str) {
        // nothing
    }

    fn set_app_id(&mut self, _slf: &Rc<XdgToplevel>, _app_id: &str) {
        // nothing
    }

    fn show_window_menu(
        &mut self,
        _slf: &Rc<XdgToplevel>,
        _seat: &Rc<WlSeat>,
        _serial: u32,
        _x: i32,
        _y: i32,
    ) {
        // nothing
    }

    fn r#move(&mut self, _slf: &Rc<XdgToplevel>, _seat: &Rc<WlSeat>, _serial: u32) {
        // nothing
    }

    fn resize(
        &mut self,
        _slf: &Rc<XdgToplevel>,
        _seat: &Rc<WlSeat>,
        _serial: u32,
        _edges: XdgToplevelResizeEdge,
    ) {
        // nothing
    }

    fn set_max_size(&mut self, _slf: &Rc<XdgToplevel>, _width: i32, _height: i32) {
        // nothing
    }

    fn set_min_size(&mut self, _slf: &Rc<XdgToplevel>, _width: i32, _height: i32) {
        // nothing
    }

    fn set_maximized(&mut self, _slf: &Rc<XdgToplevel>) {
        // nothing
    }

    fn unset_maximized(&mut self, _slf: &Rc<XdgToplevel>) {
        // nothing
    }

    fn set_fullscreen(&mut self, _slf: &Rc<XdgToplevel>, _output: Option<&Rc<WlOutput>>) {
        // nothing
    }

    fn unset_fullscreen(&mut self, _slf: &Rc<XdgToplevel>) {
        // nothing
    }

    fn set_minimized(&mut self, _slf: &Rc<XdgToplevel>) {
        // nothing
    }
}

impl ZwlrLayerSurfaceV1Handler for LayerSurfaceHandler {
    fn configure(&mut self, _slf: &Rc<ZwlrLayerSurfaceV1>, serial: u32, width: u32, height: u32) {
        let states = [
            XdgToplevelState::ACTIVATED.0,
            XdgToplevelState::FULLSCREEN.0,
            XdgToplevelState::TILED_LEFT.0,
            XdgToplevelState::TILED_TOP.0,
            XdgToplevelState::TILED_RIGHT.0,
            XdgToplevelState::TILED_BOTTOM.0,
        ];
        let _ = self
            .toplevel
            .send_configure(width as _, height as _, uapi::as_bytes(&states));
        let _ = self.xdg.xdg_surface.send_configure(serial);
    }

    fn closed(&mut self, _slf: &Rc<ZwlrLayerSurfaceV1>) {
        let _ = self.toplevel.send_close();
    }
}

struct PositionerHandler {
    anchor_rect: Option<[i32; 4]>,
}

impl XdgPositionerHandler for PositionerHandler {
    fn set_anchor_rect(
        &mut self,
        slf: &Rc<XdgPositioner>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        self.anchor_rect = Some([x, y, width, height]);
        let _ = slf.send_set_anchor_rect(x, y, width, height);
    }
}

struct XdgPopupHandlerImpl {
    parent_geometry: [i32; 4],
}

impl XdgPopupHandler for XdgPopupHandlerImpl {
    fn configure(&mut self, slf: &Rc<XdgPopup>, x: i32, y: i32, width: i32, height: i32) {
        let [dx, dy, ..] = self.parent_geometry;
        let _ = slf.send_configure(x - dx, y - dy, width, height);
    }
}
