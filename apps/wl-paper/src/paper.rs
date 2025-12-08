use {
    crate::PaperError,
    arrayvec::ArrayVec,
    std::{mem, process::Command, rc::Rc, sync::Arc},
    wl_proxy::{
        baseline::Baseline,
        global_mapper::GlobalMapper,
        object::{ConcreteObject, Object, ObjectCoreApi, ObjectRcUtils, ObjectUtils},
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
                wl_registry::{WlRegistry, WlRegistryHandler},
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
                xdg_toplevel::{XdgToplevel, XdgToplevelHandler, XdgToplevelState},
                xdg_wm_base::{XdgWmBase, XdgWmBaseHandler},
            },
        },
        simple::{SimpleCommandExt, SimpleProxy},
    },
};

pub fn main(config: Config, program: &[String]) -> Result<(), PaperError> {
    let config = Arc::new(config);
    let server = SimpleProxy::new(Baseline::V1_UNSTABLE).map_err(PaperError::CreateServer)?;
    Command::new(&program[0])
        .args(&program[1..])
        .with_wayland_display(server.display())
        .spawn_and_forward_exit_code()
        .map_err(PaperError::SpawnChild)?;
    let err = server.run(|| ClientWlDisplay {
        init: false,
        layer_shell: Default::default(),
        config: config.clone(),
        registries_without_handlers: Default::default(),
        wl_fixes: Default::default(),
    });
    Err(PaperError::ServerFailed(err))
}

pub struct Config {
    pub keyboard_interactivity: ZwlrLayerSurfaceV1KeyboardInteractivity,
    pub layer: ZwlrLayerShellV1Layer,
    pub margin_top: i32,
    pub margin_right: i32,
    pub margin_bottom: i32,
    pub margin_left: i32,
}

/// Handler for the client's wl_display object.
struct ClientWlDisplay {
    config: Arc<Config>,

    /// Whether the initial wl_registry has been created.
    init: bool,
    /// Client-created registries that don't have a handler yet because the initial
    /// roundtrip has not yet completed.
    registries_without_handlers: Vec<Rc<WlRegistry>>,

    /// The proxy-created zwlr_layer_shell_v1 object.
    layer_shell: Option<Rc<ZwlrLayerShellV1>>,
    /// The proxy-created wl_fixes object used to destroy the initial registry.
    wl_fixes: Option<Rc<WlFixes>>,
}

/// Handler for the proxy-created wl_registry object.
struct ProxyWlRegistry {
    wl_display: Rc<WlDisplay>,
}

/// Handler for the proxy-created initial wl_callback object.
struct ProxyFirstRoundtripWlCallback {
    wl_display: Rc<WlDisplay>,
    wl_registry: Rc<WlRegistry>,
}

/// Handler for client-created wl_registry objects.
struct ClientWlRegistry {
    config: Arc<Config>,
    layer_shell: Rc<ZwlrLayerShellV1>,
    filter: GlobalMapper,
    /// Whether we have sent the synthetic globals.
    init: bool,
}

/// Handler for client-created xdg_wm_base objects.
struct ClientXdgWmBase {
    config: Arc<Config>,
    zwlr_layer_shell_v1: Rc<ZwlrLayerShellV1>,
}

/// Handler for client-created xdg_surface objects.
struct ClientXdgSurface {
    config: Arc<Config>,
    zwlr_layer_shell_v1: Rc<ZwlrLayerShellV1>,
    wl_surface: Rc<WlSurface>,
    xdg_surface: Rc<XdgSurface>,
    xdg_wm_base: Rc<XdgWmBase>,
    zwlr_layer_surface_v1: Option<Rc<ZwlrLayerSurfaceV1>>,
    geometry: Option<[i32; 4]>,
    /// Whether this object has been forwarded to the server. We cannot initially forward
    /// to the server because we don't know if this will become an xdg_toplevel or an
    /// xdg_popup.
    has_server: bool,
}

/// Handler for client-created xdg_toplevel objects.
struct ClientXdgToplevel {
    /// The corresponding zwlr_layer_surface_v1.
    zwlr_layer_surface_v1: Rc<ZwlrLayerSurfaceV1>,
}

/// Handler for proxy-created zwlr_layer_surface_v1 objects.
struct ProxyZwlrLayerSurfaceV1 {
    xdg_surface: Rc<XdgSurface>,
    xdg_toplevel: Rc<XdgToplevel>,
}

/// Handler for client-created xdg_positioner objects.
struct ClientXdgPositioner {
    anchor_rect: Option<[i32; 4]>,
}

/// Handler for client-created xdg_popup objects.
struct ClientXdgPopup {
    /// The geometry of the parent xdg_client specified by the client when creating the
    /// popup. This is needed because xdg_popup.configure events for
    /// zwlr_layer_surface_v1 parents are relative to the surface whereas for xdg_surface
    /// parents they are relative to the geometry.
    parent_geometry: [i32; 4],
}

struct ClientZxdgDecorationManagerV1;

struct ClientZxdgToplevelDecorationV1;

struct ClientOrgKdeKwinServerDecorationManager;

struct ClientOrgKdeKwinServerDecoration;

impl WlDisplayHandler for ClientWlDisplay {
    fn handle_get_registry(&mut self, slf: &Rc<WlDisplay>, registry: &Rc<WlRegistry>) {
        if !self.init {
            self.init = true;
            let wl_registry = slf.create_child::<WlRegistry>();
            wl_registry.set_handler(ProxyWlRegistry {
                wl_display: slf.clone(),
            });
            slf.send_get_registry(&wl_registry);
            let sync = slf.create_child::<WlCallback>();
            sync.set_handler(ProxyFirstRoundtripWlCallback {
                wl_display: slf.clone(),
                wl_registry,
            });
            slf.send_sync(&sync);
        }
        slf.send_get_registry(registry);
        if let Some(layer_shell) = &self.layer_shell {
            registry.set_handler(ClientWlRegistry {
                init: false,
                layer_shell: layer_shell.clone(),
                config: self.config.clone(),
                filter: Default::default(),
            });
        } else {
            self.registries_without_handlers.push(registry.clone());
        }
    }
}

impl WlRegistryHandler for ProxyWlRegistry {
    fn handle_global(
        &mut self,
        slf: &Rc<WlRegistry>,
        name: u32,
        interface: ObjectInterface,
        version: u32,
    ) {
        let dh = &mut *self.wl_display.get_handler_mut::<ClientWlDisplay>();
        match interface {
            ZwlrLayerShellV1::INTERFACE => {
                let proxy = slf
                    .state()
                    .create_object::<ZwlrLayerShellV1>(version.min(5));
                slf.send_bind(name, proxy.clone());
                dh.layer_shell = Some(proxy);
            }
            WlFixes::INTERFACE => {
                let proxy = slf.state().create_object::<WlFixes>(1);
                slf.send_bind(name, proxy.clone());
                dh.wl_fixes = Some(proxy);
            }
            _ => {}
        }
    }
}

impl WlCallbackHandler for ProxyFirstRoundtripWlCallback {
    fn handle_done(&mut self, _slf: &Rc<WlCallback>, _callback_data: u32) {
        let dh = &mut *self.wl_display.get_handler_mut::<ClientWlDisplay>();
        if let Some(wl_fixes) = dh.wl_fixes.take() {
            wl_fixes.send_destroy_registry(&self.wl_registry);
            wl_fixes.send_destroy();
        }
        let Some(zwlr_layer_shell_v1) = &dh.layer_shell else {
            eprintln!(
                "Server does not support {}",
                ZwlrLayerShellV1::INTERFACE_NAME,
            );
            std::process::exit(1);
        };
        if zwlr_layer_shell_v1.version()
            < ZwlrLayerSurfaceV1::ENM__KEYBOARD_INTERACTIVITY_ON_DEMAND__SINCE
        {
            eprintln!(
                "Server does not support {} version {}",
                ZwlrLayerShellV1::INTERFACE_NAME,
                ZwlrLayerSurfaceV1::ENM__KEYBOARD_INTERACTIVITY_ON_DEMAND__SINCE,
            );
            std::process::exit(1);
        }
        for registry in mem::take(&mut dh.registries_without_handlers) {
            registry.set_handler(ClientWlRegistry {
                init: false,
                config: dh.config.clone(),
                layer_shell: zwlr_layer_shell_v1.clone(),
                filter: Default::default(),
            });
        }
    }
}

impl WlRegistryHandler for ClientWlRegistry {
    fn handle_bind(&mut self, slf: &Rc<WlRegistry>, name: u32, id: Rc<dyn Object>) {
        match id.interface() {
            XdgWmBase::INTERFACE => {
                let id = id.downcast::<XdgWmBase>();
                id.set_handler(ClientXdgWmBase {
                    zwlr_layer_shell_v1: self.layer_shell.clone(),
                    config: self.config.clone(),
                });
            }
            ZxdgDecorationManagerV1::INTERFACE => {
                let id = id.downcast::<ZxdgDecorationManagerV1>();
                id.set_handler(ClientZxdgDecorationManagerV1);
                return;
            }
            OrgKdeKwinServerDecorationManager::INTERFACE => {
                let id = id.downcast::<OrgKdeKwinServerDecorationManager>();
                id.set_handler(ClientOrgKdeKwinServerDecorationManager);
                return;
            }
            _ => {}
        }
        self.filter.forward_bind(slf, name, &id);
    }

    fn handle_global(
        &mut self,
        slf: &Rc<WlRegistry>,
        name: u32,
        interface: ObjectInterface,
        version: u32,
    ) {
        if !self.init {
            self.init = true;
            self.filter
                .add_synthetic_global(slf, ObjectInterface::ZxdgDecorationManagerV1, 1);
            self.filter.add_synthetic_global(
                slf,
                ObjectInterface::OrgKdeKwinServerDecorationManager,
                1,
            );
        }
        match interface {
            ObjectInterface::ZxdgDecorationManagerV1
            | ObjectInterface::OrgKdeKwinServerDecorationManager
            | ObjectInterface::XdgWmDialogV1
            | ObjectInterface::XdgToplevelDragManagerV1
            | ObjectInterface::XdgToplevelIconManagerV1
            | ObjectInterface::XdgToplevelTagManagerV1 => {
                self.filter.ignore_global(name);
            }
            _ => {
                self.filter.forward_global(slf, name, interface, version);
            }
        }
    }

    fn handle_global_remove(&mut self, slf: &Rc<WlRegistry>, name: u32) {
        self.filter.forward_global_remove(slf, name);
    }
}

impl XdgWmBaseHandler for ClientXdgWmBase {
    fn handle_create_positioner(&mut self, slf: &Rc<XdgWmBase>, id: &Rc<XdgPositioner>) {
        id.set_handler(ClientXdgPositioner {
            anchor_rect: Default::default(),
        });
        slf.send_create_positioner(id);
    }

    fn handle_get_xdg_surface(
        &mut self,
        slf: &Rc<XdgWmBase>,
        id: &Rc<XdgSurface>,
        surface: &Rc<WlSurface>,
    ) {
        id.set_handler(ClientXdgSurface {
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

impl XdgSurfaceHandler for ClientXdgSurface {
    fn handle_destroy(&mut self, slf: &Rc<XdgSurface>) {
        slf.unset_handler();
        if self.has_server {
            slf.send_destroy();
            self.has_server = false;
        } else {
            slf.delete_id();
        }
    }

    fn handle_get_toplevel(&mut self, _slf: &Rc<XdgSurface>, id: &Rc<XdgToplevel>) {
        let zwlr_layer_surface_v1 = self
            .zwlr_layer_shell_v1
            .create_child::<ZwlrLayerSurfaceV1>();
        let c = &*self.config;
        self.zwlr_layer_shell_v1.send_get_layer_surface(
            &zwlr_layer_surface_v1,
            &self.wl_surface,
            None,
            c.layer,
            "",
        );
        zwlr_layer_surface_v1.send_set_size(0, 0);
        zwlr_layer_surface_v1.send_set_margin(
            c.margin_top,
            c.margin_right,
            c.margin_bottom,
            c.margin_left,
        );
        zwlr_layer_surface_v1.send_set_keyboard_interactivity(c.keyboard_interactivity);
        zwlr_layer_surface_v1.send_set_anchor(
            ZwlrLayerSurfaceV1Anchor::TOP
                | ZwlrLayerSurfaceV1Anchor::BOTTOM
                | ZwlrLayerSurfaceV1Anchor::LEFT
                | ZwlrLayerSurfaceV1Anchor::RIGHT,
        );
        if id.version() >= XdgToplevel::MSG__WM_CAPABILITIES__SINCE {
            id.send_wm_capabilities(&[]);
        }
        id.set_forward_to_server(false);
        id.set_handler(ClientXdgToplevel {
            zwlr_layer_surface_v1: zwlr_layer_surface_v1.clone(),
        });
        zwlr_layer_surface_v1.set_handler(ProxyZwlrLayerSurfaceV1 {
            xdg_surface: self.xdg_surface.clone(),
            xdg_toplevel: id.clone(),
        });
        self.zwlr_layer_surface_v1 = Some(zwlr_layer_surface_v1.clone());
    }

    fn handle_get_popup(
        &mut self,
        slf: &Rc<XdgSurface>,
        id: &Rc<XdgPopup>,
        client_parent: Option<&Rc<XdgSurface>>,
        positioner: &Rc<XdgPositioner>,
    ) {
        if !self.has_server {
            self.has_server = true;
            self.xdg_wm_base.send_get_xdg_surface(slf, &self.wl_surface);
        }
        if let Some([x, y, w, h]) = self.geometry {
            slf.send_set_window_geometry(x, y, w, h);
        }
        let mut zwlr_layer_surface_v1 = None;
        let mut proxy_parent = client_parent;
        let mut parent_geometry = None;
        if let Some(parent) = client_parent
            && let Ok(client_xdg_surface) = parent.try_get_handler_ref::<ClientXdgSurface>()
            && let Some(layer_surface) = &client_xdg_surface.zwlr_layer_surface_v1
        {
            zwlr_layer_surface_v1 = Some(layer_surface.clone());
            proxy_parent = None;
            parent_geometry = client_xdg_surface.geometry;
        }
        let client_xdg_positioner = &*positioner.get_handler_ref::<ClientXdgPositioner>();
        if let Some([gx, gy, _, _]) = parent_geometry
            && let Some([px, py, pw, ph]) = client_xdg_positioner.anchor_rect
        {
            positioner.send_set_anchor_rect(gx + px, gy + py, pw, ph);
        }
        slf.send_get_popup(id, proxy_parent, positioner);
        if parent_geometry.is_some()
            && let Some([px, py, pw, ph]) = client_xdg_positioner.anchor_rect
        {
            positioner.send_set_anchor_rect(px, py, pw, ph);
        }
        if let Some(zwlr_layer_surface_v1) = zwlr_layer_surface_v1 {
            zwlr_layer_surface_v1.send_get_popup(id);
        }
        if let Some(pg) = parent_geometry {
            id.set_handler(ClientXdgPopup {
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
        self.geometry = Some([x, y, width, height]);
        if self.has_server {
            slf.send_set_window_geometry(x, y, width, height);
        }
    }

    fn handle_ack_configure(&mut self, slf: &Rc<XdgSurface>, serial: u32) {
        if let Some(tl) = &self.zwlr_layer_surface_v1 {
            tl.send_ack_configure(serial);
        } else if self.has_server {
            slf.send_ack_configure(serial);
        }
    }
}

impl XdgToplevelHandler for ClientXdgToplevel {
    fn handle_destroy(&mut self, slf: &Rc<XdgToplevel>) {
        slf.unset_handler();
        self.zwlr_layer_surface_v1.send_destroy();
    }
}

impl ZwlrLayerSurfaceV1Handler for ProxyZwlrLayerSurfaceV1 {
    fn delete_id(&mut self, slf: &Rc<ZwlrLayerSurfaceV1>) {
        slf.unset_handler();
        self.xdg_toplevel.delete_id();
    }

    fn handle_configure(
        &mut self,
        _slf: &Rc<ZwlrLayerSurfaceV1>,
        serial: u32,
        width: u32,
        height: u32,
    ) {
        let mut states = ArrayVec::<_, 9>::new();
        if self.xdg_toplevel.version() >= XdgToplevel::ENM__STATE_TILED_TOP__SINCE {
            states.extend([
                XdgToplevelState::TILED_LEFT.0,
                XdgToplevelState::TILED_TOP.0,
                XdgToplevelState::TILED_RIGHT.0,
                XdgToplevelState::TILED_BOTTOM.0,
            ]);
        }
        if self.xdg_toplevel.version() >= XdgToplevel::ENM__STATE_CONSTRAINED_TOP__SINCE {
            states.extend([
                XdgToplevelState::CONSTRAINED_LEFT.0,
                XdgToplevelState::CONSTRAINED_TOP.0,
                XdgToplevelState::CONSTRAINED_RIGHT.0,
                XdgToplevelState::CONSTRAINED_BOTTOM.0,
            ]);
        }
        if self.xdg_toplevel.version() >= XdgToplevel::MSG__CONFIGURE_BOUNDS__SINCE {
            self.xdg_toplevel
                .send_configure_bounds(width as _, height as _);
        }
        self.xdg_toplevel
            .send_configure(width as _, height as _, uapi::as_bytes(&*states));
        self.xdg_surface.send_configure(serial);
    }

    fn handle_closed(&mut self, _slf: &Rc<ZwlrLayerSurfaceV1>) {
        self.xdg_toplevel.send_close();
    }
}

impl XdgPositionerHandler for ClientXdgPositioner {
    fn handle_set_anchor_rect(
        &mut self,
        slf: &Rc<XdgPositioner>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        self.anchor_rect = Some([x, y, width, height]);
        slf.send_set_anchor_rect(x, y, width, height);
    }
}

impl XdgPopupHandler for ClientXdgPopup {
    fn handle_configure(&mut self, slf: &Rc<XdgPopup>, x: i32, y: i32, width: i32, height: i32) {
        let [dx, dy, ..] = self.parent_geometry;
        slf.send_configure(x - dx, y - dy, width, height);
    }
}

impl ZxdgDecorationManagerV1Handler for ClientZxdgDecorationManagerV1 {
    fn handle_destroy(&mut self, slf: &Rc<ZxdgDecorationManagerV1>) {
        slf.delete_id();
    }

    fn handle_get_toplevel_decoration(
        &mut self,
        _slf: &Rc<ZxdgDecorationManagerV1>,
        id: &Rc<ZxdgToplevelDecorationV1>,
        _toplevel: &Rc<XdgToplevel>,
    ) {
        id.set_handler(ClientZxdgToplevelDecorationV1);
        id.send_configure(ZxdgToplevelDecorationV1Mode::SERVER_SIDE);
    }
}

impl ZxdgToplevelDecorationV1Handler for ClientZxdgToplevelDecorationV1 {
    fn handle_destroy(&mut self, slf: &Rc<ZxdgToplevelDecorationV1>) {
        slf.delete_id();
    }

    fn handle_set_mode(
        &mut self,
        slf: &Rc<ZxdgToplevelDecorationV1>,
        _mode: ZxdgToplevelDecorationV1Mode,
    ) {
        slf.send_configure(ZxdgToplevelDecorationV1Mode::SERVER_SIDE);
    }

    fn handle_unset_mode(&mut self, slf: &Rc<ZxdgToplevelDecorationV1>) {
        slf.send_configure(ZxdgToplevelDecorationV1Mode::SERVER_SIDE);
    }
}

impl OrgKdeKwinServerDecorationManagerHandler for ClientOrgKdeKwinServerDecorationManager {
    fn handle_create(
        &mut self,
        _slf: &Rc<OrgKdeKwinServerDecorationManager>,
        id: &Rc<OrgKdeKwinServerDecoration>,
        _surface: &Rc<WlSurface>,
    ) {
        id.set_handler(ClientOrgKdeKwinServerDecoration);
        id.send_mode(OrgKdeKwinServerDecorationMode::SERVER.0);
    }
}

impl OrgKdeKwinServerDecorationHandler for ClientOrgKdeKwinServerDecoration {
    fn handle_release(&mut self, slf: &Rc<OrgKdeKwinServerDecoration>) {
        slf.delete_id();
    }

    fn handle_request_mode(&mut self, slf: &Rc<OrgKdeKwinServerDecoration>, mode: u32) {
        slf.send_mode(mode);
    }
}
