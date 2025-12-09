use {
    std::{
        any::Any,
        cell::{Cell, RefCell},
        collections::HashMap,
        rc::Rc,
    },
    wl_proxy::{
        client::Client,
        generated::{
            ProxyInterface,
            wayland::{
                wl_display::{MetaWlDisplay, MetaWlDisplayMessageHandler},
                wl_output::MetaWlOutput,
                wl_registry::{MetaWlRegistry, MetaWlRegistryMessageHandler},
                wl_seat::MetaWlSeat,
                wl_surface::MetaWlSurface,
            },
            wlr_layer_shell_unstable_v1::{
                zwlr_layer_shell_v1::{MetaZwlrLayerShellV1, MetaZwlrLayerShellV1Layer},
                zwlr_layer_surface_v1::{
                    MetaZwlrLayerSurfaceV1, MetaZwlrLayerSurfaceV1Anchor,
                    MetaZwlrLayerSurfaceV1KeyboardInteractivity,
                    MetaZwlrLayerSurfaceV1MessageHandler,
                },
            },
            xdg_shell::{
                xdg_popup::{MetaXdgPopup, MetaXdgPopupMessageHandler},
                xdg_positioner::{MetaXdgPositioner, MetaXdgPositionerMessageHandler},
                xdg_surface::{MetaXdgSurface, MetaXdgSurfaceMessageHandler},
                xdg_toplevel::{
                    MetaXdgToplevel, MetaXdgToplevelMessageHandler, MetaXdgToplevelResizeEdge,
                    MetaXdgToplevelState,
                },
                xdg_wm_base::{MetaXdgWmBase, MetaXdgWmBaseMessageHandler},
            },
        },
        proxy::Proxy,
        state::{ClientsHandler, State},
    },
};

fn main() {
    let state = State::new().unwrap();
    state.set_clients_handler(Box::new(ClientsHandlerImpl {}));
    let acceptor = state.create_acceptor().unwrap();
    eprintln!("{}", acceptor.display);
    loop {
        state.dispatch_blocking().unwrap();
    }
}

struct Shared {
    mutable: RefCell<SharedMut>,
}

struct SharedMut {
    layer_shell: Option<Rc<MetaZwlrLayerShellV1>>,
    xdg_layer_map: HashMap<u32, LayerData>,
    positioners: HashMap<u32, [i32; 4]>,
}

struct ClientsHandlerImpl {}

impl ClientsHandler for ClientsHandlerImpl {
    fn new_client(&mut self, client: &Rc<Client>) {
        let shared = Rc::new(Shared {
            mutable: RefCell::new(SharedMut {
                layer_shell: Default::default(),
                xdg_layer_map: Default::default(),
                positioners: Default::default(),
            }),
        });
        client
            .display
            .set_handler(Box::new(DisplayHandlerImpl { shared }));
    }
}

struct DisplayHandlerImpl {
    shared: Rc<Shared>,
}

impl MetaWlDisplayMessageHandler for DisplayHandlerImpl {
    fn get_registry(&mut self, slf: &Rc<MetaWlDisplay>, registry: &Rc<MetaWlRegistry>) {
        let _ = slf.send_get_registry(registry);
        registry.set_handler(Box::new(RegistryHandlerImpl {
            shared: self.shared.clone(),
        }));
    }
}

struct RegistryHandlerImpl {
    shared: Rc<Shared>,
}

impl MetaWlRegistryMessageHandler for RegistryHandlerImpl {
    fn bind(&mut self, slf: &Rc<MetaWlRegistry>, name: u32, id: Rc<dyn Proxy>) {
        if id.core().interface == ProxyInterface::XdgWmBase {
            let id = (id.clone() as Rc<dyn Any>)
                .downcast::<MetaXdgWmBase>()
                .unwrap();
            id.set_handler(Box::new(XdgWmBaseHandlerImpl {
                shared: self.shared.clone(),
            }));
        }
        let _ = slf.send_bind(name, id);
    }

    fn global(&mut self, slf: &Rc<MetaWlRegistry>, name: u32, interface: &str, version: u32) {
        if interface == ProxyInterface::ZxdgDecorationManagerV1.name() {
            return;
        }
        if interface == ProxyInterface::ZwlrLayerShellV1.name() {
            let shared = &mut *self.shared.mutable.borrow_mut();
            if shared.layer_shell.is_none() {
                let layer_shell = MetaZwlrLayerShellV1::new(&slf.core().state, version.min(5));
                let _ = slf.send_bind(name, layer_shell.clone());
                shared.layer_shell = Some(layer_shell);
            }
        }
        let _ = slf.send_global(name, interface, version);
    }
}

struct XdgWmBaseHandlerImpl {
    shared: Rc<Shared>,
}

impl MetaXdgWmBaseMessageHandler for XdgWmBaseHandlerImpl {
    fn create_positioner(&mut self, slf: &Rc<MetaXdgWmBase>, id: &Rc<MetaXdgPositioner>) {
        id.set_handler(Box::new(PositionerHandler {
            id: id.core().client_obj_id.get().unwrap(),
            shared: self.shared.clone(),
        }));
        let _ = slf.send_create_positioner(id);
    }

    fn get_xdg_surface(
        &mut self,
        slf: &Rc<MetaXdgWmBase>,
        id: &Rc<MetaXdgSurface>,
        surface: &Rc<MetaWlSurface>,
    ) {
        let shared = &mut *self.shared.mutable.borrow_mut();
        let Some(layer_shell) = &shared.layer_shell else {
            let _ = slf.send_get_xdg_surface(id, surface);
            return;
        };
        let ss = Rc::new(XdgSurfaceShared {
            layer_shell: layer_shell.clone(),
            surface: surface.clone(),
            xdg_surface: id.clone(),
            xdg_wm_base: slf.clone(),
            geometry: Default::default(),
        });
        id.set_handler(Box::new(XdgSurfaceHandler {
            shared: self.shared.clone(),
            surface: ss.clone(),
            layer_surface: None,
        }));
    }
}

struct XdgSurfaceShared {
    layer_shell: Rc<MetaZwlrLayerShellV1>,
    surface: Rc<MetaWlSurface>,
    xdg_surface: Rc<MetaXdgSurface>,
    xdg_wm_base: Rc<MetaXdgWmBase>,
    geometry: Cell<Option<[i32; 4]>>,
}

struct XdgSurfaceHandler {
    shared: Rc<Shared>,
    surface: Rc<XdgSurfaceShared>,
    layer_surface: Option<Rc<MetaZwlrLayerSurfaceV1>>,
}

struct LayerData {
    xdg: Rc<XdgSurfaceShared>,
    layer_surface: Rc<MetaZwlrLayerSurfaceV1>,
}

struct ToplevelHandler {
    shared: Rc<Shared>,
    xdg: Rc<XdgSurfaceShared>,
    layer_surface: Rc<MetaZwlrLayerSurfaceV1>,
}

struct LayerSurfaceHandler {
    shared: Rc<Shared>,
    xdg: Rc<XdgSurfaceShared>,
    toplevel: Rc<MetaXdgToplevel>,
    layer_surface: Rc<MetaZwlrLayerSurfaceV1>,
}

impl MetaXdgSurfaceMessageHandler for XdgSurfaceHandler {
    fn destroy(&mut self, slf: &Rc<MetaXdgSurface>) {
        slf.unset_handler();
        if slf.core().server_obj_id.get().is_some() {
            let _ = slf.send_destroy();
        }
    }

    fn get_toplevel(&mut self, slf: &Rc<MetaXdgSurface>, id: &Rc<MetaXdgToplevel>) {
        let layer_surface = slf
            .core()
            .state
            .create_proxy::<MetaZwlrLayerSurfaceV1>(self.surface.layer_shell.core().version);
        let _ = self.surface.layer_shell.send_get_layer_surface(
            &layer_surface,
            &self.surface.surface,
            None,
            MetaZwlrLayerShellV1Layer::BACKGROUND,
            "",
        );
        let _ = layer_surface.send_set_size(0, 0);
        // let _ = layer_surface.send_set_margin(100, 100, 100, 100);
        let _ = layer_surface.send_set_keyboard_interactivity(
            MetaZwlrLayerSurfaceV1KeyboardInteractivity::ON_DEMAND,
        );
        let _ = layer_surface.send_set_anchor(
            MetaZwlrLayerSurfaceV1Anchor::TOP
                | MetaZwlrLayerSurfaceV1Anchor::BOTTOM
                | MetaZwlrLayerSurfaceV1Anchor::LEFT
                | MetaZwlrLayerSurfaceV1Anchor::RIGHT,
        );
        id.set_handler(Box::new(ToplevelHandler {
            shared: self.shared.clone(),
            xdg: self.surface.clone(),
            layer_surface: layer_surface.clone(),
        }));
        layer_surface.set_handler(Box::new(LayerSurfaceHandler {
            shared: self.shared.clone(),
            xdg: self.surface.clone(),
            toplevel: id.clone(),
            layer_surface: layer_surface.clone(),
        }));
        self.layer_surface = Some(layer_surface.clone());
        let shared = &mut *self.shared.mutable.borrow_mut();
        shared.xdg_layer_map.insert(
            slf.core().client_obj_id.get().unwrap(),
            LayerData {
                xdg: self.surface.clone(),
                layer_surface,
            },
        );
    }

    fn get_popup(
        &mut self,
        slf: &Rc<MetaXdgSurface>,
        id: &Rc<MetaXdgPopup>,
        parent: Option<&Rc<MetaXdgSurface>>,
        positioner: &Rc<MetaXdgPositioner>,
    ) {
        let _ = self
            .surface
            .xdg_wm_base
            .send_get_xdg_surface(slf, &self.surface.surface);
        if let Some([x, y, w, h]) = self.surface.geometry.get() {
            let _ = slf.send_set_window_geometry(x, y, w, h);
        }
        let shared = &mut *self.shared.mutable.borrow_mut();
        let mut layer_parent = None;
        let mut direct_parent = parent;
        let mut parent_geometry = None;
        if let Some(parent) = parent
            && let Some(parent) = shared
                .xdg_layer_map
                .get(&parent.core().client_obj_id.get().unwrap())
        {
            layer_parent = Some(parent);
            direct_parent = None;
            parent_geometry = parent.xdg.geometry.get();
        }
        if let Some([gx, gy, _, _]) = parent_geometry
            && let Some(&[px, py, pw, ph]) = shared
                .positioners
                .get(&positioner.core().client_obj_id.get().unwrap())
        {
            let _ = positioner.send_set_anchor_rect(gx + px, gy + py, pw, ph);
        }
        let _ = slf.send_get_popup(id, direct_parent, positioner);
        if parent_geometry.is_some()
            && let Some(&[px, py, pw, ph]) = shared
                .positioners
                .get(&positioner.core().client_obj_id.get().unwrap())
        {
            let _ = positioner.send_set_anchor_rect(px, py, pw, ph);
        }
        if let Some(layer_parent) = layer_parent {
            let _ = layer_parent.layer_surface.send_get_popup(id);
        }
        if let Some(pg) = parent_geometry {
            id.set_handler(Box::new(XdgPopupHandler {
                parent_geometry: pg,
            }));
        }
    }

    fn set_window_geometry(
        &mut self,
        slf: &Rc<MetaXdgSurface>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        self.surface.geometry.set(Some([x, y, width, height]));
        if slf.core().server_obj_id.get().is_some() {
            let _ = slf.send_set_window_geometry(x, y, width, height);
        }
    }

    fn ack_configure(&mut self, slf: &Rc<MetaXdgSurface>, serial: u32) {
        if let Some(tl) = &self.layer_surface {
            let _ = tl.send_ack_configure(serial);
        } else if slf.core().server_obj_id.get().is_some() {
            let _ = slf.send_ack_configure(serial);
        }
    }
}

impl MetaXdgToplevelMessageHandler for ToplevelHandler {
    fn destroy(&mut self, slf: &Rc<MetaXdgToplevel>) {
        slf.unset_handler();
        self.layer_surface.unset_handler();
        let _ = self.layer_surface.send_destroy();
        let shared = &mut *self.shared.mutable.borrow_mut();
        shared
            .xdg_layer_map
            .remove(&self.xdg.xdg_surface.core().client_obj_id.get().unwrap());
    }

    fn set_parent(&mut self, _slf: &Rc<MetaXdgToplevel>, _parent: Option<&Rc<MetaXdgToplevel>>) {
        // nothing
    }

    fn set_title(&mut self, _slf: &Rc<MetaXdgToplevel>, _title: &str) {
        // nothing
    }

    fn set_app_id(&mut self, _slf: &Rc<MetaXdgToplevel>, _app_id: &str) {
        // nothing
    }

    fn show_window_menu(
        &mut self,
        _slf: &Rc<MetaXdgToplevel>,
        _seat: &Rc<MetaWlSeat>,
        _serial: u32,
        _x: i32,
        _y: i32,
    ) {
        // nothing
    }

    fn r#move(&mut self, _slf: &Rc<MetaXdgToplevel>, _seat: &Rc<MetaWlSeat>, _serial: u32) {
        // nothing
    }

    fn resize(
        &mut self,
        _slf: &Rc<MetaXdgToplevel>,
        _seat: &Rc<MetaWlSeat>,
        _serial: u32,
        _edges: MetaXdgToplevelResizeEdge,
    ) {
        // nothing
    }

    fn set_max_size(&mut self, _slf: &Rc<MetaXdgToplevel>, _width: i32, _height: i32) {
        // nothing
    }

    fn set_min_size(&mut self, _slf: &Rc<MetaXdgToplevel>, _width: i32, _height: i32) {
        // nothing
    }

    fn set_maximized(&mut self, _slf: &Rc<MetaXdgToplevel>) {
        // nothing
    }

    fn unset_maximized(&mut self, _slf: &Rc<MetaXdgToplevel>) {
        // nothing
    }

    fn set_fullscreen(&mut self, _slf: &Rc<MetaXdgToplevel>, _output: Option<&Rc<MetaWlOutput>>) {
        // nothing
    }

    fn unset_fullscreen(&mut self, _slf: &Rc<MetaXdgToplevel>) {
        // nothing
    }

    fn set_minimized(&mut self, _slf: &Rc<MetaXdgToplevel>) {
        // nothing
    }
}

impl MetaZwlrLayerSurfaceV1MessageHandler for LayerSurfaceHandler {
    fn configure(
        &mut self,
        _slf: &Rc<MetaZwlrLayerSurfaceV1>,
        serial: u32,
        width: u32,
        height: u32,
    ) {
        let states = [
            MetaXdgToplevelState::ACTIVATED.0,
            MetaXdgToplevelState::TILED_LEFT.0,
            MetaXdgToplevelState::TILED_TOP.0,
            MetaXdgToplevelState::TILED_RIGHT.0,
            MetaXdgToplevelState::TILED_BOTTOM.0,
        ];
        let _ = self
            .toplevel
            .send_configure(width as _, height as _, uapi::as_bytes(&states));
        let _ = self.xdg.xdg_surface.send_configure(serial);
    }

    fn closed(&mut self, _slf: &Rc<MetaZwlrLayerSurfaceV1>) {
        let _ = self.toplevel.send_close();
    }
}

struct PositionerHandler {
    id: u32,
    shared: Rc<Shared>,
}

impl MetaXdgPositionerMessageHandler for PositionerHandler {
    fn destroy(&mut self, slf: &Rc<MetaXdgPositioner>) {
        let shared = &mut *self.shared.mutable.borrow_mut();
        shared.positioners.remove(&self.id);
        let _ = slf.send_destroy();
    }

    fn set_anchor_rect(
        &mut self,
        slf: &Rc<MetaXdgPositioner>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        let shared = &mut *self.shared.mutable.borrow_mut();
        shared.positioners.insert(self.id, [x, y, width, height]);
        // let _ = slf.send_set_anchor_rect(x, y, width, height);
    }
}

struct XdgPopupHandler {
    parent_geometry: [i32; 4],
}

impl MetaXdgPopupMessageHandler for XdgPopupHandler {
    fn configure(&mut self, slf: &Rc<MetaXdgPopup>, x: i32, y: i32, width: i32, height: i32) {
        let [dx, dy, ..] = self.parent_geometry;
        let _ = slf.send_configure(x - dx, y - dy, width, height);
    }
}
