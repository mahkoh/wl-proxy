use {
    crate::WttError,
    arrayvec::ArrayVec,
    run_on_drop::on_drop,
    std::{
        cell::RefCell,
        collections::{HashMap, VecDeque},
        mem,
        process::{Command, exit},
        rc::{Rc, Weak},
    },
    wl_proxy::{
        baseline::Baseline,
        fixed::Fixed,
        global_mapper::GlobalMapper,
        object::{Object, ObjectRcUtils, ObjectUtils},
        protocols::{
            ObjectInterface,
            cursor_shape_v1::{
                wp_cursor_shape_device_v1::{WpCursorShapeDeviceV1, WpCursorShapeDeviceV1Shape},
                wp_cursor_shape_manager_v1::WpCursorShapeManagerV1,
            },
            jay_tray_v1::{
                jay_tray_item_v1::{
                    JayTrayItemV1, JayTrayItemV1Handler, JayTrayItemV1KeyboardFocusHint,
                },
                jay_tray_v1::JayTrayV1,
            },
            org_kde_kwin_server_decoration_v1::{
                org_kde_kwin_server_decoration::{
                    OrgKdeKwinServerDecoration, OrgKdeKwinServerDecorationHandler,
                    OrgKdeKwinServerDecorationMode,
                },
                org_kde_kwin_server_decoration_manager::{
                    OrgKdeKwinServerDecorationManager, OrgKdeKwinServerDecorationManagerHandler,
                },
            },
            single_pixel_buffer_v1::wp_single_pixel_buffer_manager_v1::WpSinglePixelBufferManagerV1,
            viewporter::{wp_viewport::WpViewport, wp_viewporter::WpViewporter},
            wayland::{
                wl_buffer::WlBuffer,
                wl_callback::{WlCallback, WlCallbackHandler},
                wl_compositor::{WlCompositor, WlCompositorHandler},
                wl_display::{WlDisplay, WlDisplayHandler},
                wl_keyboard::{WlKeyboard, WlKeyboardHandler, WlKeyboardKeyState},
                wl_output::{WlOutput, WlOutputTransform},
                wl_pointer::{
                    WlPointer, WlPointerAxis, WlPointerAxisRelativeDirection, WlPointerAxisSource,
                    WlPointerButtonState, WlPointerHandler,
                },
                wl_region::{WlRegion, WlRegionHandler},
                wl_registry::{WlRegistry, WlRegistryHandler},
                wl_seat::{WlSeat, WlSeatCapability, WlSeatHandler},
                wl_subcompositor::{WlSubcompositor, WlSubcompositorHandler},
                wl_subsurface::{WlSubsurface, WlSubsurfaceHandler},
                wl_surface::{WlSurface, WlSurfaceHandler},
                wl_touch::{WlTouch, WlTouchHandler},
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
                xdg_positioner::{
                    XdgPositioner, XdgPositionerAnchor, XdgPositionerConstraintAdjustment,
                    XdgPositionerGravity, XdgPositionerHandler,
                },
                xdg_surface::{XdgSurface, XdgSurfaceHandler},
                xdg_toplevel::{
                    XdgToplevel, XdgToplevelHandler, XdgToplevelResizeEdge, XdgToplevelState,
                },
                xdg_wm_base::{XdgWmBase, XdgWmBaseHandler},
            },
        },
        simple::{SimpleCommandExt, SimpleServer},
        utils::handler_holder::HandlerMut,
    },
};

pub fn main(program: &[String]) -> Result<(), WttError> {
    let server = SimpleServer::new(Baseline::V1_UNSTABLE).map_err(WttError::CreateServer)?;
    Command::new(&program[0])
        .args(&program[1..])
        .with_wayland_display(server.display())
        .spawn_and_forward_exit_code()
        .map_err(WttError::SpawnChild)?;
    let err = server.run(|| DisplayHandler {
        init: false,
        shared: Default::default(),
    });
    Err(WttError::ServerFailed(err))
}

#[derive(Default)]
struct Shared {
    shared: RefCell<SharedMut>,
}

#[derive(Default)]
struct SharedMut {
    trays: HashMap<u32, Rc<JayTrayV1>>,
    toplevels: HashMap<u64, Rc<XdgSurface>>,
    globals: Option<Rc<Globals>>,
    wl_compositor: Option<Rc<WlCompositor>>,
    wl_subcompositor: Option<Rc<WlSubcompositor>>,
    wp_viewporter: Option<Rc<WpViewporter>>,
    wp_single_pixel_buffer_manager_v1: Option<Rc<WpSinglePixelBufferManagerV1>>,
    wp_cursor_shape_manager_v1: Option<Rc<WpCursorShapeManagerV1>>,
    xdg_wm_base: Option<Rc<XdgWmBase>>,
    seats: HashMap<u32, Weak<Seat>>,
    initial_seats: HashMap<u32, u32>,
}

struct Seat {
    wl_seat: Rc<WlSeat>,
    globals: Rc<Globals>,
    mutable: RefCell<SeatMut>,
}

struct SeatMut {
    wl_pointer: Option<Rc<WlPointer>>,
}

struct DisplayHandler {
    init: bool,
    shared: Rc<Shared>,
}

impl WlDisplayHandler for DisplayHandler {
    fn handle_get_registry(&mut self, slf: &Rc<WlDisplay>, registry: &Rc<WlRegistry>) {
        if !self.init {
            self.init = true;
            let wl_registry = slf.create_child::<WlRegistry>();
            wl_registry.set_handler(FirstRegistryHandler {
                shared: self.shared.clone(),
            });
            let _ = slf.send_get_registry(&wl_registry);
            let sync = slf.create_child::<WlCallback>();
            sync.set_handler(FirstSyncHandler {
                shared: self.shared.clone(),
                registry: wl_registry,
            });
            let _ = slf.send_sync(&sync);
        }
        let mut filter = GlobalMapper::default();
        let _ = filter.add_synthetic_global(registry, ObjectInterface::XdgWmBase, 7);
        let _ = filter.add_synthetic_global(registry, ObjectInterface::ZxdgDecorationManagerV1, 1);
        let _ = filter.add_synthetic_global(registry, ObjectInterface::OrgKdeKwinServerDecorationManager, 1);
        let _ = filter.add_synthetic_global(
            registry,
            ObjectInterface::OrgKdeKwinServerDecorationManager,
            1,
        );
        let _ = slf.send_get_registry(registry);
        registry.set_handler(ClientWlRegistry {
            shared: self.shared.clone(),
            filter,
        });
    }
}

struct FirstRegistryHandler {
    shared: Rc<Shared>,
}

impl WlRegistryHandler for FirstRegistryHandler {
    fn handle_global(
        &mut self,
        slf: &Rc<WlRegistry>,
        name: u32,
        interface: ObjectInterface,
        version: u32,
    ) {
        macro_rules! bind {
            ($field:ident, $ty:ty, $min:expr) => {{
                let proxy = slf.state().create_object::<$ty>(version.min($min));
                let _ = slf.send_bind(name, proxy.clone());
                self.shared.shared.borrow_mut().$field = Some(proxy);
            }};
        }
        match interface {
            WlCompositor::INTERFACE => bind!(wl_compositor, WlCompositor, 6),
            WlSubcompositor::INTERFACE => bind!(wl_subcompositor, WlSubcompositor, 1),
            WpViewporter::INTERFACE => bind!(wp_viewporter, WpViewporter, 1),
            WpSinglePixelBufferManagerV1::INTERFACE => bind!(
                wp_single_pixel_buffer_manager_v1,
                WpSinglePixelBufferManagerV1,
                1
            ),
            WpCursorShapeManagerV1::INTERFACE => {
                bind!(wp_cursor_shape_manager_v1, WpCursorShapeManagerV1, 1)
            }
            XdgWmBase::INTERFACE => bind!(xdg_wm_base, XdgWmBase, 7),
            JayTrayV1::INTERFACE => {
                let shared = &mut *self.shared.shared.borrow_mut();
                shared.handle_new_tray(&self.shared, slf, name, version);
            }
            WlSeat::INTERFACE => {
                let shared = &mut *self.shared.shared.borrow_mut();
                shared.handle_new_seat(slf, name, version);
            }
            _ => {}
        }
    }

    fn handle_global_remove(&mut self, _slf: &Rc<WlRegistry>, name: u32) {
        let shared = &mut *self.shared.shared.borrow_mut();
        if shared.globals.is_none() {
            shared.initial_seats.remove(&name);
            return;
        }
        if let Some(seat) = shared.seats.remove(&name)
            && let Some(seat) = seat.upgrade()
        {
            let mutable = &mut *seat.mutable.borrow_mut();
            mutable.handle_capabilities(&seat, WlSeatCapability::empty());
            seat.wl_seat.unset_handler();
            let _ = seat.wl_seat.send_release();
        } else if let Some(_) = shared.trays.remove(&name) {
            for tl in shared.toplevels.values() {
                let mut xdg_surface_handler = tl.try_get_handler_mut::<ClientXdgSurface>().unwrap();
                if let Some(item) = xdg_surface_handler.jay_tray_items.remove(&name)
                    && let Some(item) = item.upgrade()
                {
                    let mut tray_handler = item
                        .try_get_handler_mut::<JayTrayItemV1HandlerImpl>()
                        .unwrap();
                    tray_handler.destroy(&item, &mut xdg_surface_handler);
                }
            }
        }
    }
}

impl SeatMut {
    fn handle_capabilities(&mut self, seat: &Rc<Seat>, capabilities: WlSeatCapability) {
        if capabilities.contains(WlSeatCapability::POINTER) {
            if self.wl_pointer.is_none() {
                let proxy = seat.wl_seat.create_child::<WlPointer>();
                let _ = seat.wl_seat.send_get_pointer(&proxy);
                let wp_cursor_shape_device_v1 =
                    seat.globals.wp_cursor_shape_manager_v1.create_child();
                let _ = seat
                    .globals
                    .wp_cursor_shape_manager_v1
                    .send_get_pointer(&wp_cursor_shape_device_v1, &proxy);
                proxy.set_handler(ProxyWlPointer {
                    seat: seat.clone(),
                    wp_cursor_shape_device_v1,
                    pointer_focus: None,
                });
                self.wl_pointer = Some(proxy);
            }
        } else {
            if let Some(proxy) = self.wl_pointer.take() {
                let mut h = proxy.try_get_handler_mut::<ProxyWlPointer>().unwrap();
                h.pointer_focus = None;
                h.wp_cursor_shape_device_v1.unset_handler();
                let _ = h.wp_cursor_shape_device_v1.send_destroy();
                proxy.unset_handler();
                let _ = proxy.send_release();
            }
        }
    }
}

impl SharedMut {
    fn handle_new_seat(&mut self, registry: &Rc<WlRegistry>, name: u32, version: u32) {
        let Some(globals) = &self.globals else {
            self.initial_seats.insert(name, version);
            return;
        };
        let proxy = registry.state().create_object::<WlSeat>(version.min(10));
        let _ = registry.send_bind(name, proxy.clone());
        let seat = Rc::new(Seat {
            wl_seat: proxy.clone(),
            globals: globals.clone(),
            mutable: RefCell::new(SeatMut { wl_pointer: None }),
        });
        proxy.set_handler(ProxyWlSeat { seat: seat.clone() });
        self.seats.insert(name, Rc::downgrade(&seat));
    }

    fn handle_new_tray(
        &mut self,
        _shared: &Rc<Shared>,
        registry: &Rc<WlRegistry>,
        name: u32,
        _version: u32,
    ) {
        let proxy = registry.state().create_object::<JayTrayV1>(1);
        let _ = registry.send_bind(name, proxy.clone());
        self.trays.insert(name, proxy.clone());
        for xdg_surface in self.toplevels.values() {
            let mut h = xdg_surface
                .try_get_handler_mut::<ClientXdgSurface>()
                .unwrap();
            if let Some(xdg_toplevel) = h.client_xdg_toplevel.upgrade() {
                h.create_tray_item(name, &proxy, xdg_surface, &xdg_toplevel);
            }
        }
    }
}

struct FirstSyncHandler {
    shared: Rc<Shared>,
    registry: Rc<WlRegistry>,
}

impl WlCallbackHandler for FirstSyncHandler {
    fn handle_done(&mut self, _slf: &Rc<WlCallback>, _callback_data: u32) {
        let shared = &mut *self.shared.shared.borrow_mut();
        macro_rules! expect {
            ($field:ident) => {
                let $field = match shared.$field.clone() {
                    Some(f) => f,
                    _ => {
                        eprintln!("Server does not support {}", stringify!($field));
                        exit(1);
                    }
                };
            };
        }
        expect!(wl_compositor);
        expect!(wl_subcompositor);
        expect!(wp_viewporter);
        expect!(wp_cursor_shape_manager_v1);
        expect!(wp_single_pixel_buffer_manager_v1);
        expect!(xdg_wm_base);
        let empty_region = wl_compositor.create_child::<WlRegion>();
        let _ = wl_compositor.send_create_region(&empty_region);
        let transparent_spb = wp_single_pixel_buffer_manager_v1.create_child::<WlBuffer>();
        let _ = wp_single_pixel_buffer_manager_v1.send_create_u32_rgba_buffer(
            &transparent_spb,
            0,
            0,
            0,
            0,
        );
        let black_spb = wp_single_pixel_buffer_manager_v1.create_child::<WlBuffer>();
        let _ = wp_single_pixel_buffer_manager_v1.send_create_u32_rgba_buffer(
            &black_spb,
            0,
            0,
            0,
            u32::MAX,
        );
        let globals = Globals {
            wl_compositor,
            wl_subcompositor,
            wp_viewporter,
            wp_cursor_shape_manager_v1,
            xdg_wm_base,
            empty_region,
            transparent_spb,
            black_spb,
        };
        shared.globals = Some(Rc::new(globals));
        for (name, version) in mem::take(&mut shared.initial_seats) {
            shared.handle_new_seat(&self.registry, name, version);
        }
    }
}

struct ClientWlRegistry {
    shared: Rc<Shared>,
    filter: GlobalMapper,
}

impl WlRegistryHandler for ClientWlRegistry {
    fn handle_bind(&mut self, slf: &Rc<WlRegistry>, name: u32, id: Rc<dyn Object>) {
        let shared = &mut *self.shared.shared.borrow_mut();
        match id.interface() {
            XdgWmBase::INTERFACE => {
                let id = id.clone().downcast::<XdgWmBase>().unwrap();
                id.set_handler(ClientXdgWmBase {
                    shared: self.shared.clone(),
                    globals: shared.globals.clone().unwrap(),
                });
                return;
            }
            WlCompositor::INTERFACE => {
                let id = id.clone().downcast::<WlCompositor>().unwrap();
                id.set_handler(ClientWlCompositor {
                    globals: shared.globals.clone().unwrap(),
                });
            }
            WlSubcompositor::INTERFACE => {
                let id = id.clone().downcast::<WlSubcompositor>().unwrap();
                id.set_handler(ClientWlSubcompositor {});
            }
            WlSeat::INTERFACE => {
                let id = id.clone().downcast::<WlSeat>().unwrap();
                id.set_handler(ClientWlSeat {});
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
            | ObjectInterface::XdgWmBase
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

struct ClientWlCompositor {
    globals: Rc<Globals>,
}

impl WlCompositorHandler for ClientWlCompositor {
    fn handle_create_surface(&mut self, slf: &Rc<WlCompositor>, id: &Rc<WlSurface>) {
        id.set_handler(ClientWlSurface {
            attached: false,
            globals: self.globals.clone(),
            pending_attach: Default::default(),
            xdg_surface: Default::default(),
            client_input_region: Default::default(),
            input_mask: Default::default(),
            subsurfaces: Default::default(),
        });
        let _ = slf.send_create_surface(id);
    }

    fn handle_create_region(&mut self, slf: &Rc<WlCompositor>, id: &Rc<WlRegion>) {
        id.set_handler(ClientWlRegion {
            ops: Default::default(),
        });
        let _ = slf.send_create_region(id);
    }
}

struct ClientWlSubcompositor {}

impl WlSubcompositorHandler for ClientWlSubcompositor {
    fn handle_get_subsurface(
        &mut self,
        slf: &Rc<WlSubcompositor>,
        id: &Rc<WlSubsurface>,
        surface: &Rc<WlSurface>,
        parent: &Rc<WlSurface>,
    ) {
        let mut p = parent.try_get_handler_mut::<ClientWlSurface>().unwrap();
        let mut c = surface.try_get_handler_mut::<ClientWlSurface>().unwrap();
        p.subsurfaces
            .insert(surface.unique_id(), ([0, 0], surface.clone()));
        c.set_input_mask(surface, p.input_mask);
        id.set_handler(ClientWlSubsurface {
            parent: parent.clone(),
            surface: surface.clone(),
        });
        let _ = slf.send_get_subsurface(id, surface, parent);
    }
}

struct ClientWlSubsurface {
    parent: Rc<WlSurface>,
    surface: Rc<WlSurface>,
}

impl WlSubsurfaceHandler for ClientWlSubsurface {
    fn handle_destroy(&mut self, slf: &Rc<WlSubsurface>) {
        self.parent
            .get_handler_mut::<ClientWlSurface>()
            .subsurfaces
            .remove(&self.surface.unique_id());
        self.surface
            .get_handler_mut::<ClientWlSurface>()
            .set_input_mask(&self.surface, None);
        slf.unset_handler();
        let _ = slf.send_destroy();
    }

    fn handle_set_position(&mut self, slf: &Rc<WlSubsurface>, dx: i32, dy: i32) {
        let mut p = self.parent.get_handler_mut::<ClientWlSurface>();
        p.subsurfaces.get_mut(&self.surface.unique_id()).unwrap().0 = [dx, dy];
        let input_mask = p.input_mask.map(|[x, y, w, h]| [x - dx, y - dy, w, h]);
        self.surface
            .get_handler_mut::<ClientWlSurface>()
            .set_input_mask(&self.surface, input_mask);
        let _ = slf.send_set_position(dx, dy);
    }
}

struct Globals {
    wl_compositor: Rc<WlCompositor>,
    wl_subcompositor: Rc<WlSubcompositor>,
    wp_viewporter: Rc<WpViewporter>,
    wp_cursor_shape_manager_v1: Rc<WpCursorShapeManagerV1>,
    xdg_wm_base: Rc<XdgWmBase>,
    empty_region: Rc<WlRegion>,
    transparent_spb: Rc<WlBuffer>,
    black_spb: Rc<WlBuffer>,
}

struct ClientWlSurface {
    attached: bool,
    globals: Rc<Globals>,
    pending_attach: Option<bool>,
    xdg_surface: Weak<XdgSurface>,
    client_input_region: Option<Vec<WlRegionOp>>,
    input_mask: Option<[i32; 4]>,
    subsurfaces: HashMap<u64, ([i32; 2], Rc<WlSurface>)>,
}

impl ClientWlSurface {
    fn set_input_mask(&mut self, slf: &Rc<WlSurface>, mask: Option<[i32; 4]>) {
        self.input_mask = mask;
        self.update_region(slf);
        for ([dx, dy], surface) in self.subsurfaces.values() {
            let mask = mask.map(|[x, y, w, h]| [x - *dx, y - *dy, w, h]);
            surface
                .try_get_handler_mut::<ClientWlSurface>()
                .unwrap()
                .set_input_mask(surface, mask);
        }
    }

    fn update_region(&self, slf: &Rc<WlSurface>) {
        let wl_region = self.globals.wl_compositor.create_child::<WlRegion>();
        let _ = self.globals.wl_compositor.send_create_region(&wl_region);
        let _destroy_region = on_drop(|| {
            let _ = wl_region.send_destroy();
        });
        let Some([x, y, w, h]) = self.input_mask else {
            let Some(region) = &self.client_input_region else {
                let _ = slf.send_set_input_region(None);
                return;
            };
            for op in region {
                match *op {
                    WlRegionOp::Add([x, y, w, h]) => {
                        let _ = wl_region.send_add(x, y, w, h);
                    }
                    WlRegionOp::Sub([x, y, w, h]) => {
                        let _ = wl_region.send_subtract(x, y, w, h);
                    }
                }
            }
            let _ = slf.send_set_input_region(Some(&wl_region));
            return;
        };
        let Some(region) = &self.client_input_region else {
            let _ = wl_region.send_add(x, y, w, h);
            let _ = slf.send_set_input_region(Some(&wl_region));
            return;
        };
        for op in region {
            match *op {
                WlRegionOp::Add([x_, y_, w_, h_]) => {
                    let x1 = x.max(x_);
                    let y1 = y.max(y_);
                    let x2 = x.saturating_add(w).min(x_.saturating_add(w_));
                    let y2 = y.saturating_add(h).min(y_.saturating_add(h_));
                    if x1 >= x2 || y1 >= y2 {
                        continue;
                    }
                    let _ = wl_region.send_add(x1, y1, x2 - x1, y2 - y1);
                }
                WlRegionOp::Sub([x, y, w, h]) => {
                    let _ = wl_region.send_subtract(x, y, w, h);
                }
            }
        }
        let _ = slf.send_set_input_region(Some(&wl_region));
    }
}

impl WlSurfaceHandler for ClientWlSurface {
    fn handle_attach(
        &mut self,
        slf: &Rc<WlSurface>,
        buffer: Option<&Rc<WlBuffer>>,
        x: i32,
        y: i32,
    ) {
        self.pending_attach = Some(buffer.is_some());
        let _ = slf.send_attach(buffer, x, y);
    }

    fn handle_set_input_region(&mut self, slf: &Rc<WlSurface>, wl_region: Option<&Rc<WlRegion>>) {
        let region = wl_region.map(|r| {
            r.try_get_handler_mut::<ClientWlRegion>()
                .unwrap()
                .ops
                .clone()
        });
        self.client_input_region = region;
        self.update_region(slf);
    }

    fn handle_commit(&mut self, slf: &Rc<WlSurface>) {
        let _ = slf.send_commit();
        let old_attached = self.attached;
        let new_attached = self.pending_attach.take().unwrap_or(old_attached);
        self.attached = new_attached;
        let Some(xdg_surface) = self.xdg_surface.upgrade() else {
            return;
        };
        let mut h = xdg_surface
            .try_get_handler_mut::<ClientXdgSurface>()
            .unwrap();
        match (old_attached, new_attached) {
            (false, false) => {
                h.handle_initial_commit(&xdg_surface);
            }
            (false, true) => {
                h.handle_map(&xdg_surface);
            }
            (true, false) => {
                h.handle_unmap();
            }
            (true, true) => {
                if h.map_upon_ack == Some(h.last_ack) {
                    h.map_upon_ack = None;
                    let _ = h
                        .client_wl_surface
                        .send_attach(Some(&h.globals.transparent_spb), 0, 0);
                    let _ = h.client_wl_surface.send_commit();
                }
            }
        }
    }
}

impl ClientXdgSurface {
    fn handle_initial_commit(&mut self, xdg_surface: &Rc<XdgSurface>) {
        if let Some(_client_xdg_popup) = self.client_xdg_popup.upgrade() {
            if self.sent_done {
                return;
            }
            let _proxy_xdg_popup = match self.proxy_xdg_popup.upgrade() {
                Some(p) => p,
                _ => {
                    let xdg_positioner = self.create_positioner();
                    let proxy_xdg_popup = self.proxy_xdg_surface.create_child::<XdgPopup>();
                    let _ = self.proxy_xdg_surface.send_get_popup(
                        &proxy_xdg_popup,
                        self.client_xdg_popup_parent.upgrade().as_ref(),
                        &xdg_positioner,
                    );
                    let _ = xdg_positioner.send_destroy();
                    self.proxy_xdg_popup = Rc::downgrade(&proxy_xdg_popup);
                    proxy_xdg_popup
                }
            };
        } else if let Some(client_xdg_toplevel) = self.client_xdg_toplevel.upgrade() {
            let states = [
                XdgToplevelState::CONSTRAINED_RIGHT.0,
                XdgToplevelState::CONSTRAINED_TOP.0,
            ];
            let _ = client_xdg_toplevel.send_configure(
                self.toplevel_size[0],
                self.toplevel_size[1],
                uapi::as_bytes(&states),
            );
            let serial = self.create_client_serial(None);
            let _ = xdg_surface.send_configure(serial as u32);
            eprintln!("sending initial configure");
        }
    }

    fn create_positioner(&self) -> Rc<XdgPositioner> {
        let pp = &self.popup_position;
        let p = self.globals.xdg_wm_base.create_child::<XdgPositioner>();
        let _ = self.globals.xdg_wm_base.send_create_positioner(&p);
        if let Some([w, h]) = pp.size {
            let _ = p.send_set_size(w, h);
        }
        if let Some([x, y, w, h]) = pp.anchor_rect {
            let _ = p.send_set_anchor_rect(x, y, w, h);
        }
        if let Some(anchor) = pp.anchor {
            let _ = p.send_set_anchor(anchor);
        }
        if let Some(gravity) = pp.gravity {
            let _ = p.send_set_gravity(gravity);
        }
        if let Some(constraint_adjustment) = pp.constraint_adjustment {
            let _ = p.send_set_constraint_adjustment(constraint_adjustment);
        }
        if let Some([x, y]) = pp.offset {
            let _ = p.send_set_offset(x, y);
        }
        if pp.reactive {
            let _ = p.send_set_reactive();
        }
        if let Some([w, h]) = pp.parent_size {
            let _ = p.send_set_parent_size(w, h);
        }
        p
    }

    fn handle_map(&mut self, xdg_surface: &Rc<XdgSurface>) {
        if let Some(xdg_toplevel) = self.client_xdg_toplevel.upgrade() {
            let shared = self.shared.clone();
            let shared = &mut *shared.shared.borrow_mut();
            for (&name, tray) in &shared.trays {
                self.create_tray_item(name, tray, xdg_surface, &xdg_toplevel);
            }
        } else if let Some(_client_xdg_popup) = self.client_xdg_popup.upgrade() {
            if self.sent_done {
                return;
            }
            // todo
        }
    }

    fn handle_unmap(&mut self) {
        self.destroy_popups_recursive();
        if self.client_xdg_toplevel.strong_count() > 0 {
            for (_, jay_tray_item_v1) in mem::take(&mut self.jay_tray_items) {
                let Some(jay_tray_item_v1) = jay_tray_item_v1.upgrade() else {
                    continue;
                };
                jay_tray_item_v1
                    .try_get_handler_mut::<JayTrayItemV1HandlerImpl>()
                    .unwrap()
                    .destroy(&jay_tray_item_v1, self);
            }
        } else if let Some(xdg_popup) = self.client_xdg_popup.upgrade() {
            if let Some(xdg_popup) = self.proxy_xdg_popup.upgrade() {
                let _ = xdg_popup.send_destroy();
                xdg_popup.unset_handler();
                self.proxy_xdg_popup = Weak::new();
            }
            self.sent_done = true;
            let _ = xdg_popup.send_popup_done();
        }
        let _ = self.proxy_surface.send_attach(None, 0, 0);
        let _ = self.proxy_surface.send_commit();
    }

    fn destroy_popups_recursive(&mut self) {
        for (_, xdg_popup) in self.popups.drain() {
            let Some(xdg_popup) = xdg_popup.upgrade() else {
                continue;
            };
            let rh = xdg_popup.try_get_handler_mut::<ClientXdgPopup>().unwrap();
            {
                let mut h = rh
                    .xdg_surface
                    .try_get_handler_mut::<ClientXdgSurface>()
                    .unwrap();
                h.destroy_popups_recursive();
                h.sent_done = true;
                h.proxy_xdg_popup = Weak::new();
                if let Some(xdg_popup) = h.client_xdg_popup.upgrade() {
                    let _ = xdg_popup.send_popup_done();
                }
            }
            let _ = xdg_popup.send_destroy();
            xdg_popup.unset_handler();
        }
    }

    fn find_tray_with_popup(&self) -> Option<Rc<JayTrayItemV1>> {
        for (_, tray) in &self.jay_tray_items {
            if let Some(tray) = tray.upgrade()
                && let Ok(h) = tray.try_get_handler_mut::<JayTrayItemV1HandlerImpl>()
                && h.proxy_xdg_popup.strong_count() > 0
            {
                return Some(tray.clone());
            }
        }
        None
    }

    fn create_tray_item(
        &mut self,
        name: u32,
        tray: &Rc<JayTrayV1>,
        xdg_surface: &Rc<XdgSurface>,
        xdg_toplevel: &Rc<XdgToplevel>,
    ) {
        let globals = &self.globals;
        let wl_surface = globals.wl_compositor.create_child::<WlSurface>();
        let _ = globals.wl_compositor.send_create_surface(&wl_surface);
        let wp_viewport = globals.wp_viewporter.create_child::<WpViewport>();
        let _ = globals
            .wp_viewporter
            .send_get_viewport(&wp_viewport, &wl_surface);
        let jay_tray_item_v1 = tray.create_child::<JayTrayItemV1>();
        let _ = tray.send_get_tray_item(&jay_tray_item_v1, &wl_surface);
        let _ = wl_surface.send_attach(Some(&globals.black_spb), 0, 0);
        let _ = wl_surface.send_commit();
        wl_surface.set_handler(TrayIconWlSurfaceHandler {
            jay_tray_item_v1: jay_tray_item_v1.clone(),
        });
        let current = JayTrayItemV1Config {
            size: [1, 1],
            preferred_anchor: XdgPositionerAnchor::BOTTOM_RIGHT,
            preferred_gravity: XdgPositionerGravity::BOTTOM_LEFT,
        };
        jay_tray_item_v1.set_handler(JayTrayItemV1HandlerImpl {
            client_xdg_surface: xdg_surface.clone(),
            client_xdg_toplevel: xdg_toplevel.clone(),
            proxy_xdg_popup: Default::default(),
            wl_surface,
            wp_viewport,
            pending: current,
            current,
        });
        self.jay_tray_items
            .insert(name, Rc::downgrade(&jay_tray_item_v1));
    }

    fn create_client_serial(&mut self, server_serial: Option<u32>) -> u64 {
        let client_serial = self.next_client_serial;
        self.next_client_serial += 1;
        self.pending_serials
            .push_back((client_serial, server_serial));
        client_serial
    }

    fn recover_client_serial(&self, client_serial: u32) -> u64 {
        let mut client_serial =
            self.next_client_serial & !(u32::MAX as u64) | (client_serial as u64);
        if client_serial >= self.next_client_serial {
            client_serial = client_serial.saturating_sub(u32::MAX as u64 + 1);
        }
        client_serial
    }
}

struct ClientXdgWmBase {
    shared: Rc<Shared>,
    globals: Rc<Globals>,
}

impl XdgWmBaseHandler for ClientXdgWmBase {
    fn handle_destroy(&mut self, slf: &Rc<XdgWmBase>) {
        slf.unset_handler();
        let _ = slf.delete_id();
    }

    fn handle_create_positioner(&mut self, _slf: &Rc<XdgWmBase>, id: &Rc<XdgPositioner>) {
        id.set_handler(ClientXdgPositioner::default());
    }

    fn handle_get_xdg_surface(
        &mut self,
        _slf: &Rc<XdgWmBase>,
        client_xdg_surface: &Rc<XdgSurface>,
        client_surface: &Rc<WlSurface>,
    ) {
        let g = &*self.globals;
        let proxy_surface = g.wl_compositor.create_child::<WlSurface>();
        let _ = g.wl_compositor.send_create_surface(&proxy_surface);
        let _ = proxy_surface.send_set_input_region(Some(&g.empty_region));
        let subsurface = g.wl_subcompositor.create_child::<WlSubsurface>();
        let _ = g
            .wl_subcompositor
            .send_get_subsurface(&subsurface, client_surface, &proxy_surface);
        let _ = subsurface.send_set_desync();
        let _ = proxy_surface.send_commit();
        let proxy_xdg_surface = g.xdg_wm_base.create_child::<XdgSurface>();
        let _ = g
            .xdg_wm_base
            .send_get_xdg_surface(&proxy_xdg_surface, &proxy_surface);
        proxy_xdg_surface.set_handler(ProxyXdgSurface {
            client_xdg_surface: client_xdg_surface.clone(),
        });
        proxy_surface.set_handler(ProxyXdgSurfaceWlSurface {
            client_wl_surface: client_surface.clone(),
            _client_xdg_surface: Rc::downgrade(&client_xdg_surface),
        });
        client_xdg_surface.set_handler(ClientXdgSurface {
            shared: self.shared.clone(),
            globals: self.globals.clone(),
            client_wl_surface: client_surface.clone(),
            client_xdg_popup: Default::default(),
            client_xdg_popup_parent: Default::default(),
            client_xdg_toplevel: Default::default(),
            proxy_surface,
            proxy_xdg_surface,
            proxy_xdg_popup: Default::default(),
            subsurface,
            jay_tray_items: Default::default(),
            popups: Default::default(),
            sent_done: false,
            toplevel_size: [800, 600],
            popup_position: Default::default(),
            last_ack: 0,
            map_unpon_configure: Default::default(),
            map_upon_ack: Default::default(),
            next_client_serial: 1,
            pending_serials: Default::default(),
        });
        let mut client_surface_handler = client_surface
            .try_get_handler_mut::<ClientWlSurface>()
            .unwrap();
        client_surface_handler.xdg_surface = Rc::downgrade(&client_xdg_surface);
    }

    fn handle_pong(&mut self, _slf: &Rc<XdgWmBase>, _serial: u32) {
        // nothing
    }
}

struct ProxyXdgSurfaceWlSurface {
    client_wl_surface: Rc<WlSurface>,
    _client_xdg_surface: Weak<XdgSurface>,
}

impl WlSurfaceHandler for ProxyXdgSurfaceWlSurface {}

struct ClientXdgSurface {
    shared: Rc<Shared>,
    globals: Rc<Globals>,
    client_wl_surface: Rc<WlSurface>,
    client_xdg_popup: Weak<XdgPopup>,
    client_xdg_popup_parent: Weak<XdgSurface>,
    client_xdg_toplevel: Weak<XdgToplevel>,
    proxy_surface: Rc<WlSurface>,
    proxy_xdg_surface: Rc<XdgSurface>,
    proxy_xdg_popup: Weak<XdgPopup>,
    subsurface: Rc<WlSubsurface>,
    jay_tray_items: HashMap<u32, Weak<JayTrayItemV1>>,
    popups: HashMap<u64, Weak<XdgPopup>>,
    sent_done: bool,
    toplevel_size: [i32; 2],
    popup_position: ClientXdgPositioner,
    last_ack: u64,
    map_unpon_configure: bool,
    map_upon_ack: Option<u64>,
    next_client_serial: u64,
    pending_serials: VecDeque<(u64, Option<u32>)>,
}

impl XdgSurfaceHandler for ClientXdgSurface {
    fn handle_destroy(&mut self, slf: &Rc<XdgSurface>) {
        let _ = self.proxy_xdg_surface.send_destroy();
        let _ = self.subsurface.send_destroy();
        self.proxy_surface.unset_handler();
        let _ = self.proxy_surface.send_destroy();
        slf.unset_handler();
        let _ = slf.delete_id();
    }

    fn handle_get_toplevel(&mut self, slf: &Rc<XdgSurface>, id: &Rc<XdgToplevel>) {
        let shared = &mut *self.shared.shared.borrow_mut();
        shared.toplevels.insert(id.unique_id(), slf.clone());
        id.set_handler(ClientXdgToplevel {
            xdg_surface: slf.clone(),
        });
        self.client_xdg_toplevel = Rc::downgrade(id);
        if id.version() >= XdgToplevel::MSG__WM_CAPABILITIES__SINCE {
            let _ = id.send_wm_capabilities(&[]);
        }
    }

    fn handle_get_popup(
        &mut self,
        slf: &Rc<XdgSurface>,
        id: &Rc<XdgPopup>,
        parent: Option<&Rc<XdgSurface>>,
        positioner: &Rc<XdgPositioner>,
    ) {
        if let Some(parent) = parent {
            self.client_xdg_popup_parent = Rc::downgrade(parent);
        }
        self.popup_position = *positioner
            .try_get_handler_mut::<ClientXdgPositioner>()
            .unwrap();
        id.set_handler(ClientXdgPopup {
            xdg_surface: slf.clone(),
        });
        self.popups.insert(id.core().unique_id(), Rc::downgrade(id));
        self.client_xdg_popup = Rc::downgrade(id);
    }

    fn handle_set_window_geometry(
        &mut self,
        _slf: &Rc<XdgSurface>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        self.client_wl_surface
            .get_handler_mut::<ClientWlSurface>()
            .set_input_mask(&self.client_wl_surface, Some([x, y, width, height]));
        let _ = self
            .proxy_xdg_surface
            .send_set_window_geometry(x, y, width, height);
    }

    fn handle_ack_configure(&mut self, _slf: &Rc<XdgSurface>, client_serial: u32) {
        let client_serial = self.recover_client_serial(client_serial);
        self.last_ack = client_serial;
        while let Some(&(expected, server_serial)) = self.pending_serials.front() {
            if expected <= client_serial {
                if let Some(server_serial) = dbg!(server_serial) {
                    let _ = self.proxy_xdg_surface.send_ack_configure(server_serial);
                }
                self.pending_serials.pop_front();
            }
        }
    }
}

struct ClientXdgToplevel {
    xdg_surface: Rc<XdgSurface>,
}

impl XdgToplevelHandler for ClientXdgToplevel {
    fn handle_destroy(&mut self, slf: &Rc<XdgToplevel>) {
        let mut h = self
            .xdg_surface
            .try_get_handler_mut::<ClientXdgSurface>()
            .unwrap();
        h.handle_unmap();
        h.client_xdg_toplevel = Weak::new();
        h.shared
            .shared
            .borrow_mut()
            .toplevels
            .remove(&slf.unique_id());
        slf.unset_handler();
        let _ = slf.delete_id();
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

#[derive(Default, Copy, Clone)]
struct ClientXdgPositioner {
    size: Option<[i32; 2]>,
    anchor_rect: Option<[i32; 4]>,
    anchor: Option<XdgPositionerAnchor>,
    gravity: Option<XdgPositionerGravity>,
    constraint_adjustment: Option<XdgPositionerConstraintAdjustment>,
    offset: Option<[i32; 2]>,
    reactive: bool,
    parent_size: Option<[i32; 2]>,
}

impl XdgPositionerHandler for ClientXdgPositioner {
    fn handle_destroy(&mut self, slf: &Rc<XdgPositioner>) {
        slf.unset_handler();
        let _ = slf.delete_id();
    }

    fn handle_set_size(&mut self, _slf: &Rc<XdgPositioner>, width: i32, height: i32) {
        self.size = Some([width, height]);
    }

    fn handle_set_anchor_rect(
        &mut self,
        _slf: &Rc<XdgPositioner>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        self.anchor_rect = Some([x, y, width, height]);
    }

    fn handle_set_anchor(&mut self, _slf: &Rc<XdgPositioner>, anchor: XdgPositionerAnchor) {
        self.anchor = Some(anchor);
    }

    fn handle_set_gravity(&mut self, _slf: &Rc<XdgPositioner>, gravity: XdgPositionerGravity) {
        self.gravity = Some(gravity);
    }

    fn handle_set_constraint_adjustment(
        &mut self,
        _slf: &Rc<XdgPositioner>,
        constraint_adjustment: XdgPositionerConstraintAdjustment,
    ) {
        self.constraint_adjustment = Some(constraint_adjustment);
    }

    fn handle_set_offset(&mut self, _slf: &Rc<XdgPositioner>, x: i32, y: i32) {
        self.offset = Some([x, y]);
    }

    fn handle_set_reactive(&mut self, _slf: &Rc<XdgPositioner>) {
        self.reactive = true;
    }

    fn handle_set_parent_size(
        &mut self,
        _slf: &Rc<XdgPositioner>,
        parent_width: i32,
        parent_height: i32,
    ) {
        self.parent_size = Some([parent_width, parent_height]);
    }

    fn handle_set_parent_configure(&mut self, _slf: &Rc<XdgPositioner>, _serial: u32) {
        // nothing
    }
}

struct ClientXdgPopup {
    xdg_surface: Rc<XdgSurface>,
}

impl ClientXdgPopup {
    fn get_handler(&self) -> HandlerMut<'_, ClientXdgSurface> {
        self.xdg_surface.try_get_handler_mut().unwrap()
    }
}

impl XdgPopupHandler for ClientXdgPopup {
    fn handle_destroy(&mut self, slf: &Rc<XdgPopup>) {
        let mut h = self.get_handler();
        h.handle_unmap();
        h.client_xdg_popup = Weak::new();
        h.client_xdg_popup_parent = Weak::new();
        slf.unset_handler();
        let _ = slf.delete_id();
    }

    fn handle_grab(&mut self, _slf: &Rc<XdgPopup>, seat: &Rc<WlSeat>, serial: u32) {
        let h = self.get_handler();
        if let Some(id) = h.proxy_xdg_popup.upgrade() {
            let _ = id.send_grab(seat, serial);
        }
    }

    fn handle_reposition(
        &mut self,
        _slf: &Rc<XdgPopup>,
        positioner: &Rc<XdgPositioner>,
        token: u32,
    ) {
        let h = self.get_handler();
        if let Some(id) = h.proxy_xdg_popup.upgrade() {
            let _ = id.send_reposition(positioner, token);
        }
    }
}

#[derive(Clone)]
struct ProxyWlSeat {
    seat: Rc<Seat>,
}

impl WlSeatHandler for ProxyWlSeat {
    fn handle_capabilities(&mut self, _slf: &Rc<WlSeat>, capabilities: WlSeatCapability) {
        let mutable = &mut *self.seat.mutable.borrow_mut();
        mutable.handle_capabilities(&self.seat, capabilities);
    }
}

#[derive(Copy, Clone)]
enum WlRegionOp {
    Add([i32; 4]),
    Sub([i32; 4]),
}

struct ClientWlRegion {
    ops: Vec<WlRegionOp>,
}

impl WlRegionHandler for ClientWlRegion {
    fn handle_add(&mut self, slf: &Rc<WlRegion>, x: i32, y: i32, width: i32, height: i32) {
        self.ops.push(WlRegionOp::Add([x, y, width, height]));
        let _ = slf.send_add(x, y, width, height);
    }

    fn handle_subtract(&mut self, slf: &Rc<WlRegion>, x: i32, y: i32, width: i32, height: i32) {
        self.ops.push(WlRegionOp::Sub([x, y, width, height]));
        let _ = slf.send_subtract(x, y, width, height);
    }
}

struct ProxyWlPointer {
    seat: Rc<Seat>,
    wp_cursor_shape_device_v1: Rc<WpCursorShapeDeviceV1>,
    pointer_focus: Option<Rc<JayTrayItemV1>>,
}

impl WlPointerHandler for ProxyWlPointer {
    fn handle_enter(
        &mut self,
        _slf: &Rc<WlPointer>,
        serial: u32,
        surface: &Rc<WlSurface>,
        _surface_x: Fixed,
        _surface_y: Fixed,
    ) {
        let Ok(tsh) = surface.try_get_handler_mut::<TrayIconWlSurfaceHandler>() else {
            return;
        };
        let _ = self
            .wp_cursor_shape_device_v1
            .send_set_shape(serial, WpCursorShapeDeviceV1Shape::DEFAULT);
        self.pointer_focus = Some(tsh.jay_tray_item_v1.clone());
    }

    fn handle_leave(&mut self, _slf: &Rc<WlPointer>, _serial: u32, surface: &Rc<WlSurface>) {
        let Ok(_) = surface.try_get_handler_mut::<TrayIconWlSurfaceHandler>() else {
            return;
        };
        self.pointer_focus = None;
    }

    fn handle_motion(
        &mut self,
        _slf: &Rc<WlPointer>,
        _time: u32,
        _surface_x: Fixed,
        _surface_y: Fixed,
    ) {
        // nothing
    }

    fn handle_button(
        &mut self,
        _slf: &Rc<WlPointer>,
        serial: u32,
        _time: u32,
        _button: u32,
        state: WlPointerButtonState,
    ) {
        if state != WlPointerButtonState::PRESSED {
            return;
        }
        let Some(pf) = &self.pointer_focus else {
            return;
        };
        eprintln!("Handling button {}", pf.unique_id());
        let tray_item_handler = &mut *pf
            .try_get_handler_mut::<JayTrayItemV1HandlerImpl>()
            .unwrap();
        let client_xdg_surface = tray_item_handler.client_xdg_surface.clone();
        let xdg_surface_handler = &mut *client_xdg_surface
            .try_get_handler_mut::<ClientXdgSurface>()
            .unwrap();
        if tray_item_handler.proxy_xdg_popup.strong_count() > 0 {
            tray_item_handler.destroy_popup(xdg_surface_handler);
            eprintln!("Destroying old popup");
            return;
        }
        if let Some(other) = xdg_surface_handler.find_tray_with_popup() {
            other
                .try_get_handler_mut::<JayTrayItemV1HandlerImpl>()
                .unwrap()
                .destroy_popup(xdg_surface_handler);
        }
        eprintln!("Creating new popup");
        let config = tray_item_handler.current;
        let globals = &xdg_surface_handler.globals;
        let xdg_positioner = globals.xdg_wm_base.create_child::<XdgPositioner>();
        let _ = globals.xdg_wm_base.send_create_positioner(&xdg_positioner);
        let _ = xdg_positioner.send_set_reactive();
        let _ = xdg_positioner.send_set_anchor_rect(0, 0, config.size[0], config.size[1]);
        let _ = xdg_positioner.send_set_anchor(config.preferred_anchor);
        let _ = xdg_positioner.send_set_gravity(config.preferred_gravity);
        let _ = xdg_positioner.send_set_size(
            xdg_surface_handler.toplevel_size[0],
            xdg_surface_handler.toplevel_size[1],
        );
        let _ = xdg_positioner.send_set_constraint_adjustment(
            XdgPositionerConstraintAdjustment::FLIP_X
                | XdgPositionerConstraintAdjustment::FLIP_Y
                | XdgPositionerConstraintAdjustment::SLIDE_X
                | XdgPositionerConstraintAdjustment::SLIDE_Y
                | XdgPositionerConstraintAdjustment::RESIZE_X
                | XdgPositionerConstraintAdjustment::RESIZE_Y,
        );
        let xdg_popup = xdg_surface_handler
            .proxy_xdg_surface
            .create_child::<XdgPopup>();
        let _ =
            xdg_surface_handler
                .proxy_xdg_surface
                .send_get_popup(&xdg_popup, None, &xdg_positioner);
        let _ = xdg_positioner.send_destroy();
        let _ = pf.send_get_popup(
            &xdg_popup,
            &self.seat.wl_seat,
            serial,
            JayTrayItemV1KeyboardFocusHint::IMMEDIATE,
        );
        let _ = xdg_surface_handler.proxy_surface.send_commit();
        xdg_popup.set_handler(ProxyTrayXdgPopup {
            jay_tray_item_v1: pf.clone(),
        });
        tray_item_handler.proxy_xdg_popup = Rc::downgrade(&xdg_popup);
        xdg_surface_handler.map_unpon_configure = true;
    }

    fn handle_axis(
        &mut self,
        _slf: &Rc<WlPointer>,
        _time: u32,
        _axis: WlPointerAxis,
        _value: Fixed,
    ) {
        // nothing
    }

    fn handle_release(&mut self, _slf: &Rc<WlPointer>) {
        // nothing
    }

    fn handle_frame(&mut self, _slf: &Rc<WlPointer>) {
        // nothing
    }

    fn handle_axis_source(&mut self, _slf: &Rc<WlPointer>, _axis_source: WlPointerAxisSource) {
        // nothing
    }

    fn handle_axis_stop(&mut self, _slf: &Rc<WlPointer>, _time: u32, _axis: WlPointerAxis) {
        // nothing
    }

    fn handle_axis_discrete(&mut self, _slf: &Rc<WlPointer>, _axis: WlPointerAxis, _discrete: i32) {
        // nothing
    }

    fn handle_axis_value120(&mut self, _slf: &Rc<WlPointer>, _axis: WlPointerAxis, _value120: i32) {
        // nothing
    }

    fn handle_axis_relative_direction(
        &mut self,
        _slf: &Rc<WlPointer>,
        _axis: WlPointerAxis,
        _direction: WlPointerAxisRelativeDirection,
    ) {
        // nothing
    }
}

struct ClientWlSeat {}

impl WlSeatHandler for ClientWlSeat {
    fn handle_get_pointer(&mut self, slf: &Rc<WlSeat>, id: &Rc<WlPointer>) {
        id.set_handler(ClientWlDevice::default());
        let _ = slf.send_get_pointer(id);
    }

    fn handle_get_keyboard(&mut self, slf: &Rc<WlSeat>, id: &Rc<WlKeyboard>) {
        id.set_handler(ClientWlDevice::default());
        let _ = slf.send_get_keyboard(id);
    }

    fn handle_get_touch(&mut self, slf: &Rc<WlSeat>, id: &Rc<WlTouch>) {
        id.set_handler(ClientWlDevice::default());
        let _ = slf.send_get_touch(id);
    }
}

#[derive(Default)]
struct ClientWlDevice {
    on_client_surface: bool,
}

impl WlPointerHandler for ClientWlDevice {
    fn handle_enter(
        &mut self,
        slf: &Rc<WlPointer>,
        serial: u32,
        surface: &Rc<WlSurface>,
        surface_x: Fixed,
        surface_y: Fixed,
    ) {
        self.on_client_surface = surface.try_get_handler_mut::<ClientWlSurface>().is_ok();
        if self.on_client_surface {
            let _ = slf.send_enter(serial, surface, surface_x, surface_y);
        }
    }

    fn handle_leave(&mut self, slf: &Rc<WlPointer>, serial: u32, surface: &Rc<WlSurface>) {
        if self.on_client_surface {
            let _ = slf.send_leave(serial, surface);
        }
    }

    fn handle_motion(
        &mut self,
        slf: &Rc<WlPointer>,
        time: u32,
        surface_x: Fixed,
        surface_y: Fixed,
    ) {
        if self.on_client_surface {
            let _ = slf.send_motion(time, surface_x, surface_y);
        }
    }

    fn handle_button(
        &mut self,
        slf: &Rc<WlPointer>,
        serial: u32,
        time: u32,
        button: u32,
        state: WlPointerButtonState,
    ) {
        if self.on_client_surface {
            let _ = slf.send_button(serial, time, button, state);
        }
    }

    fn handle_axis(&mut self, slf: &Rc<WlPointer>, time: u32, axis: WlPointerAxis, value: Fixed) {
        if self.on_client_surface {
            let _ = slf.send_axis(time, axis, value);
        }
    }

    fn handle_frame(&mut self, slf: &Rc<WlPointer>) {
        let _ = slf.send_frame();
    }

    fn handle_axis_source(&mut self, slf: &Rc<WlPointer>, axis_source: WlPointerAxisSource) {
        if self.on_client_surface {
            let _ = slf.send_axis_source(axis_source);
        }
    }

    fn handle_axis_stop(&mut self, slf: &Rc<WlPointer>, time: u32, axis: WlPointerAxis) {
        if self.on_client_surface {
            let _ = slf.send_axis_stop(time, axis);
        }
    }

    fn handle_axis_discrete(&mut self, slf: &Rc<WlPointer>, axis: WlPointerAxis, discrete: i32) {
        if self.on_client_surface {
            let _ = slf.send_axis_discrete(axis, discrete);
        }
    }

    fn handle_axis_value120(&mut self, slf: &Rc<WlPointer>, axis: WlPointerAxis, value120: i32) {
        if self.on_client_surface {
            let _ = slf.send_axis_value120(axis, value120);
        }
    }

    fn handle_axis_relative_direction(
        &mut self,
        slf: &Rc<WlPointer>,
        axis: WlPointerAxis,
        direction: WlPointerAxisRelativeDirection,
    ) {
        if self.on_client_surface {
            let _ = slf.send_axis_relative_direction(axis, direction);
        }
    }
}

impl WlTouchHandler for ClientWlDevice {
    fn handle_down(
        &mut self,
        slf: &Rc<WlTouch>,
        serial: u32,
        time: u32,
        surface: &Rc<WlSurface>,
        id: i32,
        x: Fixed,
        y: Fixed,
    ) {
        self.on_client_surface = surface.try_get_handler_mut::<ClientWlSurface>().is_ok();
        if self.on_client_surface {
            let _ = slf.send_down(serial, time, surface, id, x, y);
        }
    }

    fn handle_up(&mut self, slf: &Rc<WlTouch>, serial: u32, time: u32, id: i32) {
        if self.on_client_surface {
            let _ = slf.send_up(serial, time, id);
        }
    }

    fn handle_motion(&mut self, slf: &Rc<WlTouch>, time: u32, id: i32, x: Fixed, y: Fixed) {
        if self.on_client_surface {
            let _ = slf.send_motion(time, id, x, y);
        }
    }

    fn handle_frame(&mut self, slf: &Rc<WlTouch>) {
        let _ = slf.send_frame();
    }

    fn handle_cancel(&mut self, slf: &Rc<WlTouch>) {
        if self.on_client_surface {
            let _ = slf.send_cancel();
        }
    }

    fn handle_shape(&mut self, slf: &Rc<WlTouch>, id: i32, major: Fixed, minor: Fixed) {
        if self.on_client_surface {
            let _ = slf.send_shape(id, major, minor);
        }
    }

    fn handle_orientation(&mut self, slf: &Rc<WlTouch>, id: i32, orientation: Fixed) {
        if self.on_client_surface {
            let _ = slf.send_orientation(id, orientation);
        }
    }
}

impl WlKeyboardHandler for ClientWlDevice {
    fn handle_enter(
        &mut self,
        slf: &Rc<WlKeyboard>,
        serial: u32,
        surface: &Rc<WlSurface>,
        keys: &[u8],
    ) {
        self.on_client_surface = false;
        let Ok(surface_handler) = surface.try_get_handler_mut::<ProxyXdgSurfaceWlSurface>() else {
            return;
        };
        self.on_client_surface = true;
        let _ = slf.send_enter(serial, &surface_handler.client_wl_surface, keys);
    }

    fn handle_leave(&mut self, slf: &Rc<WlKeyboard>, serial: u32, surface: &Rc<WlSurface>) {
        if self.on_client_surface {
            let _ = slf.send_leave(serial, surface);
        }
    }

    fn handle_key(
        &mut self,
        slf: &Rc<WlKeyboard>,
        serial: u32,
        time: u32,
        key: u32,
        state: WlKeyboardKeyState,
    ) {
        if self.on_client_surface {
            let _ = slf.send_key(serial, time, key, state);
        }
    }
}

struct ProxyXdgSurface {
    client_xdg_surface: Rc<XdgSurface>,
}

impl XdgSurfaceHandler for ProxyXdgSurface {
    fn handle_configure(&mut self, _slf: &Rc<XdgSurface>, server_serial: u32) {
        let mut h = self
            .client_xdg_surface
            .try_get_handler_mut::<ClientXdgSurface>()
            .unwrap();
        let client_serial = h.create_client_serial(Some(server_serial));
        let _ = self.client_xdg_surface.send_configure(client_serial as u32);
        if h.map_unpon_configure {
            h.map_unpon_configure = false;
            h.map_upon_ack = Some(client_serial);
            {
                h.map_upon_ack = None;
                let _ = h
                    .proxy_surface
                    .send_attach(Some(&h.globals.transparent_spb), 0, 0);
                let _ = h.proxy_surface.send_commit();
                eprintln!("commit");
            }
        }
    }
}

struct ProxyTrayXdgPopup {
    jay_tray_item_v1: Rc<JayTrayItemV1>,
}

impl XdgPopupHandler for ProxyTrayXdgPopup {
    fn handle_configure(&mut self, _slf: &Rc<XdgPopup>, _x: i32, _y: i32, width: i32, height: i32) {
        let tray_item_handler = self
            .jay_tray_item_v1
            .try_get_handler_mut::<JayTrayItemV1HandlerImpl>()
            .unwrap();
        let states = compute_toplevel_states(&tray_item_handler.client_xdg_toplevel);
        let _ = tray_item_handler.client_xdg_toplevel.send_configure(
            width,
            height,
            uapi::as_bytes(&*states),
        );
        let mut h = tray_item_handler
            .client_xdg_surface
            .try_get_handler_mut::<ClientXdgSurface>()
            .unwrap();
        h.toplevel_size[0] = width;
        h.toplevel_size[1] = height;
    }

    fn handle_popup_done(&mut self, _slf: &Rc<XdgPopup>) {
        let mut tray_item_handler = self
            .jay_tray_item_v1
            .try_get_handler_mut::<JayTrayItemV1HandlerImpl>()
            .unwrap();
        let client_xdg_surface = tray_item_handler.client_xdg_surface.clone();
        let mut client_xdg_surface = client_xdg_surface
            .try_get_handler_mut::<ClientXdgSurface>()
            .unwrap();
        tray_item_handler.destroy_popup(&mut client_xdg_surface);
    }
}

fn compute_toplevel_states(xdg_toplevel: &XdgToplevel) -> ArrayVec<u32, 13> {
    let mut states = ArrayVec::<u32, 13>::new();
    if xdg_toplevel.version() >= XdgToplevel::ENM__STATE_TILED_TOP__SINCE {
        states.extend([
            XdgToplevelState::TILED_TOP.0,
            XdgToplevelState::TILED_RIGHT.0,
            XdgToplevelState::TILED_BOTTOM.0,
            XdgToplevelState::TILED_LEFT.0,
        ]);
    }
    if xdg_toplevel.version() >= XdgToplevel::ENM__STATE_CONSTRAINED_TOP__SINCE {
        states.extend([
            XdgToplevelState::CONSTRAINED_TOP.0,
            XdgToplevelState::CONSTRAINED_RIGHT.0,
            XdgToplevelState::CONSTRAINED_BOTTOM.0,
            XdgToplevelState::CONSTRAINED_LEFT.0,
        ]);
    }
    states
}

struct TrayIconWlSurfaceHandler {
    jay_tray_item_v1: Rc<JayTrayItemV1>,
}

impl WlSurfaceHandler for TrayIconWlSurfaceHandler {
    fn handle_enter(&mut self, _slf: &Rc<WlSurface>, _output: &Rc<WlOutput>) {
        // nothing
    }

    fn handle_leave(&mut self, _slf: &Rc<WlSurface>, _output: &Rc<WlOutput>) {
        // nothing
    }

    fn handle_preferred_buffer_scale(&mut self, _slf: &Rc<WlSurface>, _factor: i32) {
        // nothing
    }

    fn handle_preferred_buffer_transform(
        &mut self,
        _slf: &Rc<WlSurface>,
        _transform: WlOutputTransform,
    ) {
        // nothing
    }
}

#[derive(Copy, Clone)]
struct JayTrayItemV1Config {
    size: [i32; 2],
    preferred_anchor: XdgPositionerAnchor,
    preferred_gravity: XdgPositionerGravity,
}

struct JayTrayItemV1HandlerImpl {
    // client toplevel
    client_xdg_surface: Rc<XdgSurface>,
    client_xdg_toplevel: Rc<XdgToplevel>,

    // popup containing the toplevel
    proxy_xdg_popup: Weak<XdgPopup>,

    // icon data
    wl_surface: Rc<WlSurface>,
    wp_viewport: Rc<WpViewport>,
    pending: JayTrayItemV1Config,
    current: JayTrayItemV1Config,
}

impl JayTrayItemV1HandlerImpl {
    fn destroy_popup(&mut self, client_xdg_surface: &mut ClientXdgSurface) {
        if let Some(popup) = self.proxy_xdg_popup.upgrade() {
            self.proxy_xdg_popup = Weak::new();
            client_xdg_surface.destroy_popups_recursive();
            popup.unset_handler();
            let _ = popup.send_destroy();
        }
    }

    fn destroy(&mut self, slf: &Rc<JayTrayItemV1>, client_xdg_surface: &mut ClientXdgSurface) {
        self.destroy_popup(client_xdg_surface);
        slf.unset_handler();
        self.wl_surface.unset_handler();
        let _ = slf.send_destroy();
        let _ = self.wp_viewport.send_destroy();
        let _ = self.wl_surface.send_destroy();
    }
}

impl JayTrayItemV1Handler for JayTrayItemV1HandlerImpl {
    fn handle_configure_size(&mut self, _slf: &Rc<JayTrayItemV1>, width: i32, height: i32) {
        self.pending.size = [width, height];
    }

    fn handle_preferred_anchor(&mut self, _slf: &Rc<JayTrayItemV1>, anchor: XdgPositionerAnchor) {
        self.pending.preferred_anchor = anchor;
    }

    fn handle_preferred_gravity(
        &mut self,
        _slf: &Rc<JayTrayItemV1>,
        gravity: XdgPositionerGravity,
    ) {
        self.pending.preferred_gravity = gravity;
    }

    fn handle_configure(&mut self, slf: &Rc<JayTrayItemV1>, serial: u32) {
        let _ = slf.send_ack_configure(serial);
        if self.pending.size != self.current.size {
            let _ = self
                .wp_viewport
                .send_set_destination(self.pending.size[0], self.pending.size[1]);
        }
        self.current = self.pending;
        let _ = self.wl_surface.send_commit();
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
