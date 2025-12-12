use {
    crate::PaperError,
    std::{
        cell::{Cell, RefCell},
        process::Command,
        rc::Rc,
        sync::Arc,
    },
    wl_proxy::{
        global_filter::GlobalFilter,
        object::{Object, ObjectRcUtils, ObjectUtils},
        protocols::{
            ObjectInterface,
            org_kde_kwin_server_decoration_v1::{
                org_kde_kwin_server_decoration::{
                    OrgKdeKwinServerDecoration, OrgKdeKwinServerDecorationHandler,
                    OrgKdeKwinServerDecorationMode,
                },
                org_kde_kwin_server_decoration_manager::{
                    OrgKdeKwinServerDecorationManager, OrgKdeKwinServerDecorationManagerHandler,
                },
            },
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
            xdg_decoration_unstable_v1::{
                zxdg_decoration_manager_v1::{
                    ZxdgDecorationManagerV1, ZxdgDecorationManagerV1Handler,
                },
                zxdg_toplevel_decoration_v1::{
                    ZxdgToplevelDecorationV1, ZxdgToplevelDecorationV1Handler,
                    ZxdgToplevelDecorationV1Mode,
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
    fn handle_get_registry(&mut self, slf: &Rc<WlDisplay>, registry: &Rc<WlRegistry>) {
        if !self.init {
            self.init = true;
            let wl_registry = slf.create_child::<WlRegistry>();
            wl_registry.set_handler(FirstRegistryHandler {
                layer_shell: self.layer_shell.clone(),
                wl_fixes: Default::default(),
            });
            let _ = slf.send_get_registry(&wl_registry);
            let sync = slf.create_child::<WlCallback>();
            sync.set_handler(FirstSyncHandler {
                registry: wl_registry,
            });
            let _ = slf.send_sync(&sync);
        }
        let mut filter = GlobalFilter::baseline_i_am_prototyping();
        let _ = filter.add_synthetic_global(registry, ObjectInterface::ZxdgDecorationManagerV1, 1);
        let _ = filter.add_synthetic_global(
            registry,
            ObjectInterface::OrgKdeKwinServerDecorationManager,
            1,
        );
        let _ = slf.send_get_registry(registry);
        registry.set_handler(WlRegistryHandlerImpl {
            layer_shell: self.layer_shell.clone(),
            config: self.config.clone(),
            filter,
        });
    }
}

struct FirstRegistryHandler {
    layer_shell: Rc<LayerShell>,
    wl_fixes: Option<Rc<WlFixes>>,
}

impl WlRegistryHandler for FirstRegistryHandler {
    fn handle_global(
        &mut self,
        slf: &Rc<WlRegistry>,
        name: u32,
        interface: ObjectInterface,
        version: u32,
    ) {
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
    fn handle_done(&mut self, _slf: &Rc<WlCallback>, _callback_data: u32) {
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
                ZwlrLayerShellV1::INTERFACE_NAME,
            );
            std::process::exit(1);
        }
    }
}

struct WlRegistryHandlerImpl {
    config: Arc<Config>,
    layer_shell: Rc<LayerShell>,
    filter: GlobalFilter,
}

impl WlRegistryHandler for WlRegistryHandlerImpl {
    fn handle_bind(&mut self, slf: &Rc<WlRegistry>, name: u32, id: Rc<dyn Object>) {
        match id.interface() {
            XdgWmBase::INTERFACE => {
                let id = id.clone().downcast::<XdgWmBase>().unwrap();
                id.set_handler(XdgWmBaseHandlerImpl {
                    zwlr_layer_shell_v1: self.layer_shell.layer_shell.borrow().clone().unwrap(),
                    config: self.config.clone(),
                });
            }
            ZxdgDecorationManagerV1::INTERFACE => {
                let id = id.clone().downcast::<ZxdgDecorationManagerV1>().unwrap();
                id.set_handler(ZxdgDecorationManagerV1HandlerImpl);
                return;
            }
            OrgKdeKwinServerDecorationManager::INTERFACE => {
                let id = id
                    .clone()
                    .downcast::<OrgKdeKwinServerDecorationManager>()
                    .unwrap();
                id.set_handler(OrgKdeKwinServerDecorationManagerHandlerImpl);
                return;
            }
            _ => {}
        }
        let _ = self.filter.handle_client_bind(slf, name, &id);
    }

    fn handle_global(
        &mut self,
        slf: &Rc<WlRegistry>,
        name: u32,
        interface: ObjectInterface,
        version: u32,
    ) {
        match interface {
            ObjectInterface::ZxdgDecorationManagerV1
            | ObjectInterface::OrgKdeKwinServerDecorationManager
            | ObjectInterface::XdgWmDialogV1
            | ObjectInterface::XdgToplevelDragManagerV1
            | ObjectInterface::XdgToplevelIconManagerV1
            | ObjectInterface::XdgToplevelTagManagerV1 => {
                self.filter.ignore_server_global(name);
            }
            _ => {
                let _ = self
                    .filter
                    .handle_server_global(slf, name, interface, version);
            }
        }
    }

    fn handle_global_remove(&mut self, slf: &Rc<WlRegistry>, name: u32) {
        let _ = self.filter.handle_server_global_remove(slf, name);
    }
}

struct XdgWmBaseHandlerImpl {
    config: Arc<Config>,
    zwlr_layer_shell_v1: Rc<ZwlrLayerShellV1>,
}

impl XdgWmBaseHandler for XdgWmBaseHandlerImpl {
    fn handle_create_positioner(&mut self, slf: &Rc<XdgWmBase>, id: &Rc<XdgPositioner>) {
        id.set_handler(XdgPositionerHandlerImpl {
            anchor_rect: Default::default(),
        });
        let _ = slf.send_create_positioner(id);
    }

    fn handle_get_xdg_surface(
        &mut self,
        slf: &Rc<XdgWmBase>,
        id: &Rc<XdgSurface>,
        surface: &Rc<WlSurface>,
    ) {
        id.set_handler(XdgSurfaceHandlerImpl {
            zwlr_layer_shell_v1: self.zwlr_layer_shell_v1.clone(),
            wl_surface: surface.clone(),
            xdg_surface: id.clone(),
            xdg_wm_base: slf.clone(),
            geometry: Default::default(),
            zwlr_layer_surface_v1: None,
            has_server: false,
            config: self.config.clone(),
        });
    }
}

struct XdgSurfaceHandlerImpl {
    config: Arc<Config>,
    zwlr_layer_shell_v1: Rc<ZwlrLayerShellV1>,
    wl_surface: Rc<WlSurface>,
    xdg_surface: Rc<XdgSurface>,
    xdg_wm_base: Rc<XdgWmBase>,
    zwlr_layer_surface_v1: Option<Rc<ZwlrLayerSurfaceV1>>,
    geometry: Cell<Option<[i32; 4]>>,
    has_server: bool,
}

impl XdgSurfaceHandler for XdgSurfaceHandlerImpl {
    fn handle_destroy(&mut self, slf: &Rc<XdgSurface>) {
        slf.unset_handler();
        if self.has_server {
            let _ = slf.send_destroy();
            self.has_server = false;
        } else {
            let _ = slf.delete_id();
        }
    }

    fn handle_get_toplevel(&mut self, _slf: &Rc<XdgSurface>, id: &Rc<XdgToplevel>) {
        let zwlr_layer_surface_v1 = self
            .zwlr_layer_shell_v1
            .create_child::<ZwlrLayerSurfaceV1>();
        let _ = self.zwlr_layer_shell_v1.send_get_layer_surface(
            &zwlr_layer_surface_v1,
            &self.wl_surface,
            None,
            ZwlrLayerShellV1Layer::BACKGROUND,
            "",
        );
        let c = &*self.config;
        let _ = zwlr_layer_surface_v1.send_set_size(0, 0);
        let _ = zwlr_layer_surface_v1.send_set_margin(
            c.margin_top,
            c.margin_right,
            c.margin_bottom,
            c.margin_left,
        );
        let _ = zwlr_layer_surface_v1
            .send_set_keyboard_interactivity(ZwlrLayerSurfaceV1KeyboardInteractivity::ON_DEMAND);
        let _ = zwlr_layer_surface_v1.send_set_anchor(
            ZwlrLayerSurfaceV1Anchor::TOP
                | ZwlrLayerSurfaceV1Anchor::BOTTOM
                | ZwlrLayerSurfaceV1Anchor::LEFT
                | ZwlrLayerSurfaceV1Anchor::RIGHT,
        );
        if id.version() >= XdgToplevel::MSG__WM_CAPABILITIES__SINCE {
            let _ = id.send_wm_capabilities(&[]);
        }
        id.set_handler(XdgToplevelHandlerImpl {
            zwlr_layer_surface_v1: zwlr_layer_surface_v1.clone(),
        });
        zwlr_layer_surface_v1.set_handler(ZwlrLayerSurfaceV1HandlerImpl {
            xdg_surface: self.xdg_surface.clone(),
            xdg_toplevel: id.clone(),
        });
        self.zwlr_layer_surface_v1 = Some(zwlr_layer_surface_v1.clone());
    }

    fn handle_get_popup(
        &mut self,
        slf: &Rc<XdgSurface>,
        id: &Rc<XdgPopup>,
        parent: Option<&Rc<XdgSurface>>,
        positioner: &Rc<XdgPositioner>,
    ) {
        if !self.has_server {
            self.has_server = true;
            let _ = self.xdg_wm_base.send_get_xdg_surface(slf, &self.wl_surface);
        }
        if let Some([x, y, w, h]) = self.geometry.get() {
            let _ = slf.send_set_window_geometry(x, y, w, h);
        }
        let mut layer_parent = None;
        let mut direct_parent = parent;
        let mut parent_geometry = None;
        if let Some(parent) = parent
            && let Ok(handler) = parent.get_handler_ref::<XdgSurfaceHandlerImpl>()
            && let Some(parent) = &handler.zwlr_layer_surface_v1
        {
            layer_parent = Some(parent.clone());
            direct_parent = None;
            parent_geometry = handler.geometry.get();
        }
        let pos_handler = &*positioner
            .get_handler_ref::<XdgPositionerHandlerImpl>()
            .unwrap();
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

    fn handle_set_window_geometry(
        &mut self,
        slf: &Rc<XdgSurface>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        self.geometry.set(Some([x, y, width, height]));
        if self.has_server {
            let _ = slf.send_set_window_geometry(x, y, width, height);
        }
    }

    fn handle_ack_configure(&mut self, slf: &Rc<XdgSurface>, serial: u32) {
        if let Some(tl) = &self.zwlr_layer_surface_v1 {
            let _ = tl.send_ack_configure(serial);
        } else if self.has_server {
            let _ = slf.send_ack_configure(serial);
        }
    }
}

struct XdgToplevelHandlerImpl {
    zwlr_layer_surface_v1: Rc<ZwlrLayerSurfaceV1>,
}

impl XdgToplevelHandler for XdgToplevelHandlerImpl {
    fn handle_destroy(&mut self, slf: &Rc<XdgToplevel>) {
        slf.unset_handler();
        let _ = self.zwlr_layer_surface_v1.send_destroy();
    }

    fn handle_set_parent(&mut self, _slf: &Rc<XdgToplevel>, _parent: Option<&Rc<XdgToplevel>>) {
        // nothing
    }

    fn handle_set_title(&mut self, _slf: &Rc<XdgToplevel>, _title: &str) {
        // nothing
    }

    fn handle_set_app_id(&mut self, _slf: &Rc<XdgToplevel>, _app_id: &str) {
        // nothing
    }

    fn handle_show_window_menu(
        &mut self,
        _slf: &Rc<XdgToplevel>,
        _seat: &Rc<WlSeat>,
        _serial: u32,
        _x: i32,
        _y: i32,
    ) {
        // nothing
    }

    fn handle_move(&mut self, _slf: &Rc<XdgToplevel>, _seat: &Rc<WlSeat>, _serial: u32) {
        // nothing
    }

    fn handle_resize(
        &mut self,
        _slf: &Rc<XdgToplevel>,
        _seat: &Rc<WlSeat>,
        _serial: u32,
        _edges: XdgToplevelResizeEdge,
    ) {
        // nothing
    }

    fn handle_set_max_size(&mut self, _slf: &Rc<XdgToplevel>, _width: i32, _height: i32) {
        // nothing
    }

    fn handle_set_min_size(&mut self, _slf: &Rc<XdgToplevel>, _width: i32, _height: i32) {
        // nothing
    }

    fn handle_set_maximized(&mut self, _slf: &Rc<XdgToplevel>) {
        // nothing
    }

    fn handle_unset_maximized(&mut self, _slf: &Rc<XdgToplevel>) {
        // nothing
    }

    fn handle_set_fullscreen(&mut self, _slf: &Rc<XdgToplevel>, _output: Option<&Rc<WlOutput>>) {
        // nothing
    }

    fn handle_unset_fullscreen(&mut self, _slf: &Rc<XdgToplevel>) {
        // nothing
    }

    fn handle_set_minimized(&mut self, _slf: &Rc<XdgToplevel>) {
        // nothing
    }
}

struct ZwlrLayerSurfaceV1HandlerImpl {
    xdg_surface: Rc<XdgSurface>,
    xdg_toplevel: Rc<XdgToplevel>,
}

impl ZwlrLayerSurfaceV1Handler for ZwlrLayerSurfaceV1HandlerImpl {
    fn delete_id(&mut self, slf: &Rc<ZwlrLayerSurfaceV1>) {
        slf.unset_handler();
        let _ = self.xdg_toplevel.delete_id();
    }

    fn handle_configure(
        &mut self,
        _slf: &Rc<ZwlrLayerSurfaceV1>,
        serial: u32,
        width: u32,
        height: u32,
    ) {
        let states = [
            XdgToplevelState::ACTIVATED.0,
            XdgToplevelState::TILED_LEFT.0,
            XdgToplevelState::TILED_TOP.0,
            XdgToplevelState::TILED_RIGHT.0,
            XdgToplevelState::TILED_BOTTOM.0,
        ];
        if self.xdg_toplevel.version() >= XdgToplevel::MSG__CONFIGURE_BOUNDS__SINCE {
            let _ = self
                .xdg_toplevel
                .send_configure_bounds(width as _, height as _);
        }
        let _ = self
            .xdg_toplevel
            .send_configure(width as _, height as _, uapi::as_bytes(&states));
        let _ = self.xdg_surface.send_configure(serial);
    }

    fn handle_closed(&mut self, _slf: &Rc<ZwlrLayerSurfaceV1>) {
        let _ = self.xdg_toplevel.send_close();
    }
}

struct XdgPositionerHandlerImpl {
    anchor_rect: Option<[i32; 4]>,
}

impl XdgPositionerHandler for XdgPositionerHandlerImpl {
    fn handle_set_anchor_rect(
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
    fn handle_configure(&mut self, slf: &Rc<XdgPopup>, x: i32, y: i32, width: i32, height: i32) {
        let [dx, dy, ..] = self.parent_geometry;
        let _ = slf.send_configure(x - dx, y - dy, width, height);
    }
}

struct ZxdgDecorationManagerV1HandlerImpl;

impl ZxdgDecorationManagerV1Handler for ZxdgDecorationManagerV1HandlerImpl {
    fn handle_destroy(&mut self, slf: &Rc<ZxdgDecorationManagerV1>) {
        let _ = slf.delete_id();
    }

    fn handle_get_toplevel_decoration(
        &mut self,
        _slf: &Rc<ZxdgDecorationManagerV1>,
        id: &Rc<ZxdgToplevelDecorationV1>,
        _toplevel: &Rc<XdgToplevel>,
    ) {
        id.set_handler(ZxdgToplevelDecorationV1HandlerImpl);
        let _ = id.send_configure(ZxdgToplevelDecorationV1Mode::SERVER_SIDE);
    }
}

struct ZxdgToplevelDecorationV1HandlerImpl;

impl ZxdgToplevelDecorationV1Handler for ZxdgToplevelDecorationV1HandlerImpl {
    fn handle_destroy(&mut self, slf: &Rc<ZxdgToplevelDecorationV1>) {
        let _ = slf.delete_id();
    }

    fn handle_set_mode(
        &mut self,
        slf: &Rc<ZxdgToplevelDecorationV1>,
        _mode: ZxdgToplevelDecorationV1Mode,
    ) {
        let _ = slf.send_configure(ZxdgToplevelDecorationV1Mode::SERVER_SIDE);
    }

    fn handle_unset_mode(&mut self, slf: &Rc<ZxdgToplevelDecorationV1>) {
        let _ = slf.send_configure(ZxdgToplevelDecorationV1Mode::SERVER_SIDE);
    }
}

struct OrgKdeKwinServerDecorationManagerHandlerImpl;

impl OrgKdeKwinServerDecorationManagerHandler for OrgKdeKwinServerDecorationManagerHandlerImpl {
    fn handle_create(
        &mut self,
        _slf: &Rc<OrgKdeKwinServerDecorationManager>,
        id: &Rc<OrgKdeKwinServerDecoration>,
        _surface: &Rc<WlSurface>,
    ) {
        id.set_handler(OrgKdeKwinServerDecorationHandlerImpl);
        let _ = id.send_mode(OrgKdeKwinServerDecorationMode::SERVER.0);
    }
}

struct OrgKdeKwinServerDecorationHandlerImpl;

impl OrgKdeKwinServerDecorationHandler for OrgKdeKwinServerDecorationHandlerImpl {
    fn handle_release(&mut self, slf: &Rc<OrgKdeKwinServerDecoration>) {
        let _ = slf.delete_id();
    }

    fn handle_request_mode(&mut self, slf: &Rc<OrgKdeKwinServerDecoration>, mode: u32) {
        let _ = slf.send_mode(mode);
    }
}
