#![expect(clippy::collapsible_else_if, clippy::collapsible_if)]

use {
    crate::WttError,
    arrayvec::ArrayVec,
    isnt::std_1::primitive::IsntSliceExt,
    linearize::{Linearize, StaticMap},
    run_on_drop::on_drop,
    std::{
        collections::{HashMap, VecDeque},
        io::Write,
        mem,
        os::fd::OwnedFd,
        process::{Command, exit},
        rc::Rc,
    },
    uapi::c,
    wl_proxy::{
        baseline::Baseline,
        fixed::Fixed,
        global_mapper::GlobalMapper,
        object::{ConcreteObject, Object, ObjectRcUtils, ObjectUtils},
        protocols::{
            ObjectInterface,
            cursor_shape_v1::{
                wp_cursor_shape_device_v1::{WpCursorShapeDeviceV1, WpCursorShapeDeviceV1Shape},
                wp_cursor_shape_manager_v1::WpCursorShapeManagerV1,
            },
            jay_popup_ext_v1::{
                jay_popup_ext_manager_v1::JayPopupExtManagerV1, jay_popup_ext_v1::JayPopupExtV1,
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
            relative_pointer_unstable_v1::{
                zwp_relative_pointer_manager_v1::{
                    ZwpRelativePointerManagerV1, ZwpRelativePointerManagerV1Handler,
                },
                zwp_relative_pointer_v1::{ZwpRelativePointerV1, ZwpRelativePointerV1Handler},
            },
            single_pixel_buffer_v1::wp_single_pixel_buffer_manager_v1::WpSinglePixelBufferManagerV1,
            text_input_unstable_v3::{
                zwp_text_input_manager_v3::{ZwpTextInputManagerV3, ZwpTextInputManagerV3Handler},
                zwp_text_input_v3::{ZwpTextInputV3, ZwpTextInputV3Handler},
            },
            viewporter::{wp_viewport::WpViewport, wp_viewporter::WpViewporter},
            wayland::{
                wl_buffer::{WlBuffer, WlBufferHandler},
                wl_callback::{WlCallback, WlCallbackHandler},
                wl_compositor::{WlCompositor, WlCompositorHandler},
                wl_display::{WlDisplay, WlDisplayHandler},
                wl_keyboard::{WlKeyboard, WlKeyboardHandler, WlKeyboardKeyState},
                wl_output::WlOutputTransform,
                wl_pointer::{
                    WlPointer, WlPointerAxis, WlPointerAxisRelativeDirection, WlPointerAxisSource,
                    WlPointerButtonState, WlPointerHandler,
                },
                wl_region::{WlRegion, WlRegionHandler},
                wl_registry::{WlRegistry, WlRegistryHandler},
                wl_seat::{WlSeat, WlSeatCapability, WlSeatHandler},
                wl_shm::{WlShm, WlShmFormat, WlShmHandler},
                wl_shm_pool::{WlShmPool, WlShmPoolHandler},
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
                    XdgPositionerGravity,
                },
                xdg_surface::{XdgSurface, XdgSurfaceHandler},
                xdg_toplevel::{
                    XdgToplevel, XdgToplevelHandler, XdgToplevelResizeEdge, XdgToplevelState,
                },
                xdg_wm_base::{XdgWmBase, XdgWmBaseHandler},
            },
            xdg_toplevel_icon_v1::{
                xdg_toplevel_icon_manager_v1::{
                    XdgToplevelIconManagerV1, XdgToplevelIconManagerV1Handler,
                },
                xdg_toplevel_icon_v1::{XdgToplevelIconV1, XdgToplevelIconV1Handler},
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
    let err = server.run(DisplayHandler::default);
    Err(WttError::ServerFailed(err))
}

#[derive(Default)]
struct DisplayHandler {
    registry: Option<Rc<WlRegistry>>,
    cached_requests: Vec<CachedRequest>,
    trays: HashMap<u32, Rc<JayTrayV1>>,
    toplevels: HashMap<u64, Rc<XdgSurface>>,
    globals: Option<Rc<Globals>>,
    seats: HashMap<u32, Rc<WlSeat>>,
    initial_seats: HashMap<u32, u32>,

    wl_compositor: Option<Rc<WlCompositor>>,
    wl_shm: Option<Rc<WlShm>>,
    wl_subcompositor: Option<Rc<WlSubcompositor>>,
    wp_viewporter: Option<Rc<WpViewporter>>,
    wp_single_pixel_buffer_manager_v1: Option<Rc<WpSinglePixelBufferManagerV1>>,
    wp_cursor_shape_manager_v1: Option<Rc<WpCursorShapeManagerV1>>,
    xdg_wm_base: Option<Rc<XdgWmBase>>,
    jay_popup_ext_manager_v1: Option<Rc<JayPopupExtManagerV1>>,
}

enum CachedRequest {
    GetRegistry(Rc<WlRegistry>),
    Sync(Rc<WlCallback>),
}

struct Globals {
    wl_compositor: Rc<WlCompositor>,
    wl_shm: Rc<WlShm>,
    wl_subcompositor: Rc<WlSubcompositor>,
    wp_viewporter: Rc<WpViewporter>,
    wp_cursor_shape_manager_v1: Rc<WpCursorShapeManagerV1>,
    xdg_wm_base: Rc<XdgWmBase>,
    jay_popup_ext_manager_v1: Rc<JayPopupExtManagerV1>,
    empty_region: Rc<WlRegion>,
    transparent_spb: Rc<WlBuffer>,
    black_spb: Rc<WlBuffer>,
}

struct ProxyWlRegistry {
    wl_display: Rc<WlDisplay>,
}

struct FirstSyncHandler {
    wl_display: Rc<WlDisplay>,
    wl_registry: Rc<WlRegistry>,
}

struct ProxyXdgWmBase;

struct ClientWlRegistry {
    init: bool,
    wl_display: Rc<WlDisplay>,
    globals: Rc<Globals>,
    mapper: GlobalMapper,
}

struct ClientWlCompositor {
    globals: Rc<Globals>,
}

struct ClientWlSubcompositor;

struct ClientWlSubsurface {
    parent: Rc<WlSurface>,
    surface: Rc<WlSurface>,
}

struct ClientWlSurface {
    attached: bool,
    globals: Rc<Globals>,
    pending_attach: Option<bool>,
    xdg_surface: Option<Rc<XdgSurface>>,
    client_input_region: Option<Vec<WlRegionOp>>,
    input_mask: Option<[i32; 4]>,
    subsurface_position: [i32; 2],
    subsurfaces: HashMap<u64, Rc<WlSurface>>,
}

struct ClientXdgWmBase {
    wl_display: Rc<WlDisplay>,
    globals: Rc<Globals>,
}

struct ProxyXdgSurfaceWlSurface {
    client_wl_surface: Rc<WlSurface>,
}

struct ClientXdgSurface {
    wl_display: Rc<WlDisplay>,
    globals: Rc<Globals>,
    client_wl_surface: Rc<WlSurface>,
    client_xdg_popup: Option<Rc<XdgPopup>>,
    client_xdg_toplevel: Option<Rc<XdgToplevel>>,
    proxy_wl_surface: Rc<WlSurface>,
    proxy_xdg_surface: Rc<XdgSurface>,
    proxy_xdg_popup: Option<Rc<XdgPopup>>,
    subsurface: Rc<WlSubsurface>,
    popup_jay_tray_item: Option<Rc<JayTrayItemV1>>,
    jay_tray_items: HashMap<u32, Rc<JayTrayItemV1>>,
    client_popups: HashMap<u64, Rc<XdgPopup>>,
    geometry: [i32; 4],
    pending_geometry: Option<[i32; 4]>,
    map_unpon_configure: bool,
    map_unpon_ack: Option<u64>,
    next_client_serial: u64,
    pending_serials: VecDeque<(u64, Option<u32>)>,
    needed_tray_edges: usize,
    tray_popup_borders: Option<StaticMap<WindowEdge, TrayPopupBorder>>,
    toplevel_icon: Option<Vec<Icon>>,
    last_configure_size: Option<[i32; 2]>,
    ignore_configure: bool,
}

struct Icon {
    logical_size: i32,
    buffer_size: i32,
    buffer: Rc<WlBuffer>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Linearize)]
enum WindowEdge {
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    TopLeft,
}

impl WindowEdge {
    fn to_wl(self) -> XdgToplevelResizeEdge {
        match self {
            WindowEdge::Top => XdgToplevelResizeEdge::TOP,
            WindowEdge::TopRight => XdgToplevelResizeEdge::TOP_RIGHT,
            WindowEdge::Right => XdgToplevelResizeEdge::RIGHT,
            WindowEdge::BottomRight => XdgToplevelResizeEdge::BOTTOM_RIGHT,
            WindowEdge::Bottom => XdgToplevelResizeEdge::BOTTOM,
            WindowEdge::BottomLeft => XdgToplevelResizeEdge::BOTTOM_LEFT,
            WindowEdge::Left => XdgToplevelResizeEdge::LEFT,
            WindowEdge::TopLeft => XdgToplevelResizeEdge::TOP_LEFT,
        }
    }
}

struct TrayPopupBorder {
    wl_surface: Rc<WlSurface>,
    wl_subsurface: Rc<WlSubsurface>,
    buffer_size: [usize; 2],
}

struct ProxyClientPopupXdgPopup {
    xdg_surface: Rc<XdgSurface>,
    client_xdg_popup: Rc<XdgPopup>,
}

struct ProxyTrayPopupBorderWlSurface {
    client_xdg_surface: Rc<XdgSurface>,
    window_edge: WindowEdge,
}

struct ClientXdgToplevel {
    xdg_surface: Rc<XdgSurface>,
}

struct ClientXdgPopup {
    xdg_surface: Rc<XdgSurface>,
    parent_xdg_surface: Option<Rc<XdgSurface>>,
}

#[derive(Clone)]
struct ProxyWlSeat {
    wl_seat: Rc<WlSeat>,
    globals: Rc<Globals>,
    wl_pointer: Option<Rc<WlPointer>>,
}

#[derive(Copy, Clone)]
enum WlRegionOp {
    Add([i32; 4]),
    Sub([i32; 4]),
}

struct ClientWlRegion {
    ops: Vec<WlRegionOp>,
}

struct ProxyWlPointer {
    globals: Rc<Globals>,
    seat: Rc<WlSeat>,
    wp_cursor_shape_device_v1: Rc<WpCursorShapeDeviceV1>,
    tray_icon_focus: Option<Rc<JayTrayItemV1>>,
    edge_focus: Option<WindowEdge>,
    surface: Option<Rc<WlSurface>>,
}

struct ClientWlSeat;

#[derive(Default)]
struct ClientWlSeatDevice {
    on_client_surface: bool,
}

struct ClientZwpTextInputManagerV3;

struct ClientZwpRelativePointerManagerV1;

struct ClientZwpRelativePointerV1 {
    wl_pointer: Rc<WlPointer>,
}

struct ProxyXdgSurface {
    client_xdg_surface: Rc<XdgSurface>,
}

struct ProxyTrayXdgPopup {
    jay_tray_item_v1: Rc<JayTrayItemV1>,
    jay_popup_ext_v1: Rc<JayPopupExtV1>,
}

struct ProxyTrayIconWlSurface {
    jay_tray_item_v1: Rc<JayTrayItemV1>,
}

#[derive(Copy, Clone)]
struct JayTrayItemV1Config {
    size: [i32; 2],
    preferred_anchor: XdgPositionerAnchor,
    preferred_gravity: XdgPositionerGravity,
}

struct ProxyJayTrayItemV1 {
    globals: Rc<Globals>,

    // client toplevel
    client_xdg_surface: Rc<XdgSurface>,
    client_xdg_toplevel: Rc<XdgToplevel>,

    // popup containing the toplevel
    proxy_xdg_popup: Option<Rc<XdgPopup>>,

    // icon data
    wl_surface: Rc<WlSurface>,
    wp_viewport: Rc<WpViewport>,
    pending: JayTrayItemV1Config,
    current: JayTrayItemV1Config,
}

struct ClientZxdgDecorationManagerV1;

struct ClientZxdgToplevelDecorationV1;

struct ClientOrgKdeKwinServerDecorationManager;

struct ClientOrgKdeKwinServerDecoration;

struct ClientXdgToplevelIconManagerV1 {
    globals: Rc<Globals>,
}

struct ClientXdgToplevelIconV1 {
    buffers: HashMap<(i32, i32), Rc<WlBuffer>>,
}

struct ClientWlShm;

struct ClientWlShmPool {
    fd: Rc<OwnedFd>,
    size: i32,
}

struct ClientShmWlBuffer {
    pool_fd: Rc<OwnedFd>,
    pool_size: i32,
    offset: i32,
    size: [i32; 2],
    stride: i32,
    format: WlShmFormat,
}

impl WlDisplayHandler for DisplayHandler {
    fn handle_sync(&mut self, slf: &Rc<WlDisplay>, callback: &Rc<WlCallback>) {
        if self.globals.is_some() {
            slf.send_sync(callback);
        } else {
            self.cached_requests
                .push(CachedRequest::Sync(callback.clone()));
        }
    }

    fn handle_get_registry(&mut self, slf: &Rc<WlDisplay>, registry: &Rc<WlRegistry>) {
        if self.registry.is_none() {
            let wl_registry = slf.new_send_get_registry();
            wl_registry.set_handler(ProxyWlRegistry {
                wl_display: slf.clone(),
            });
            let sync = slf.new_send_sync();
            sync.set_handler(FirstSyncHandler {
                wl_display: slf.clone(),
                wl_registry: wl_registry.clone(),
            });
            self.registry = Some(wl_registry);
        }
        if let Some(globals) = &self.globals {
            slf.send_get_registry(registry);
            registry.set_handler(ClientWlRegistry {
                init: false,
                wl_display: slf.clone(),
                globals: globals.clone(),
                mapper: Default::default(),
            });
        } else {
            self.cached_requests
                .push(CachedRequest::GetRegistry(registry.clone()));
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
        let display = &mut *self.wl_display.get_handler_mut::<DisplayHandler>();
        macro_rules! bind {
            ($field:ident, $ty:ty, $min:expr) => {{
                let proxy = slf.state().create_object::<$ty>(version.min($min));
                proxy.set_forward_to_client(false);
                slf.send_bind(name, proxy.clone());
                display.$field = Some(proxy);
            }};
        }
        match interface {
            WlCompositor::INTERFACE => bind!(wl_compositor, WlCompositor, 6),
            WlShm::INTERFACE => bind!(wl_shm, WlShm, 2),
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
            JayPopupExtManagerV1::INTERFACE => {
                bind!(jay_popup_ext_manager_v1, JayPopupExtManagerV1, 1)
            }
            XdgWmBase::INTERFACE => bind!(xdg_wm_base, XdgWmBase, 7),
            JayTrayV1::INTERFACE => display.handle_new_tray(slf, name),
            WlSeat::INTERFACE => display.handle_new_seat(slf, name, version),
            _ => {}
        }
    }

    fn handle_global_remove(&mut self, _slf: &Rc<WlRegistry>, name: u32) {
        let display = &mut *self.wl_display.get_handler_mut::<DisplayHandler>();
        if display.globals.is_none() {
            display.initial_seats.remove(&name);
            return;
        }
        if let Some(seat) = display.seats.remove(&name) {
            let proxy_wl_seat = &mut *seat.get_handler_mut::<ProxyWlSeat>();
            proxy_wl_seat.handle_capabilities(WlSeatCapability::empty());
            seat.unset_handler();
            seat.send_release();
        } else if display.trays.remove(&name).is_some() {
            for tl in display.toplevels.values() {
                let xdg_surface_handler = &mut *tl.get_handler_mut::<ClientXdgSurface>();
                if let Some(item) = xdg_surface_handler.jay_tray_items.remove(&name) {
                    let tray_handler = &mut *item.get_handler_mut::<ProxyJayTrayItemV1>();
                    tray_handler.destroy(&item, xdg_surface_handler);
                }
            }
        }
    }
}

impl DisplayHandler {
    fn handle_new_seat(&mut self, registry: &Rc<WlRegistry>, name: u32, version: u32) {
        let Some(globals) = &self.globals else {
            self.initial_seats.insert(name, version);
            return;
        };
        let proxy = registry.state().create_object::<WlSeat>(version.min(10));
        proxy.set_forward_to_client(false);
        registry.send_bind(name, proxy.clone());
        proxy.set_handler(ProxyWlSeat {
            wl_seat: proxy.clone(),
            globals: globals.clone(),
            wl_pointer: None,
        });
        self.seats.insert(name, proxy);
    }

    fn handle_new_tray(&mut self, registry: &Rc<WlRegistry>, name: u32) {
        let proxy = registry.state().create_object::<JayTrayV1>(1);
        registry.send_bind(name, proxy.clone());
        self.trays.insert(name, proxy.clone());
        for xdg_surface in self.toplevels.values() {
            let client_xdg_surface = &mut *xdg_surface.get_handler_mut::<ClientXdgSurface>();
            if let Some(xdg_toplevel) = client_xdg_surface.client_xdg_toplevel.clone() {
                client_xdg_surface.create_tray_item(name, &proxy, xdg_surface, &xdg_toplevel);
            }
        }
    }
}

impl WlCallbackHandler for FirstSyncHandler {
    fn handle_done(&mut self, _slf: &Rc<WlCallback>, _callback_data: u32) {
        let display = &mut *self.wl_display.get_handler_mut::<DisplayHandler>();
        macro_rules! expect {
            ($field:ident) => {
                let $field = match display.$field.clone() {
                    Some(f) => f,
                    _ => {
                        eprintln!("Server does not support {}", stringify!($field));
                        exit(1);
                    }
                };
            };
        }
        expect!(wl_compositor);
        expect!(wl_shm);
        expect!(wl_subcompositor);
        expect!(wp_viewporter);
        expect!(wp_cursor_shape_manager_v1);
        expect!(wp_single_pixel_buffer_manager_v1);
        expect!(xdg_wm_base);
        expect!(jay_popup_ext_manager_v1);
        let empty_region = wl_compositor.new_send_create_region();
        let transparent_spb =
            wp_single_pixel_buffer_manager_v1.new_send_create_u32_rgba_buffer(0, 0, 0, 0);
        transparent_spb.set_forward_to_client(false);
        let black_spb =
            wp_single_pixel_buffer_manager_v1.new_send_create_u32_rgba_buffer(0, 0, 0, u32::MAX);
        black_spb.set_forward_to_client(false);
        xdg_wm_base.set_handler(ProxyXdgWmBase);
        let globals = Globals {
            wl_compositor,
            wl_shm,
            wl_subcompositor,
            wp_viewporter,
            wp_cursor_shape_manager_v1,
            xdg_wm_base,
            jay_popup_ext_manager_v1,
            empty_region,
            transparent_spb,
            black_spb,
        };
        display.globals = Some(Rc::new(globals));
        for request in mem::take(&mut display.cached_requests) {
            match request {
                CachedRequest::GetRegistry(r) => display.handle_get_registry(&self.wl_display, &r),
                CachedRequest::Sync(s) => display.handle_sync(&self.wl_display, &s),
            }
        }
        for (name, version) in mem::take(&mut display.initial_seats) {
            display.handle_new_seat(&self.wl_registry, name, version);
        }
    }
}

impl XdgWmBaseHandler for ProxyXdgWmBase {
    fn handle_ping(&mut self, slf: &Rc<XdgWmBase>, serial: u32) {
        slf.send_ping(serial);
    }
}

impl WlRegistryHandler for ClientWlRegistry {
    fn handle_bind(&mut self, slf: &Rc<WlRegistry>, name: u32, id: Rc<dyn Object>) {
        match id.interface() {
            XdgWmBase::INTERFACE => {
                id.set_forward_to_server(false);
                id.downcast::<XdgWmBase>().set_handler(ClientXdgWmBase {
                    wl_display: self.wl_display.clone(),
                    globals: self.globals.clone(),
                });
                return;
            }
            WlCompositor::INTERFACE => {
                id.downcast::<WlCompositor>()
                    .set_handler(ClientWlCompositor {
                        globals: self.globals.clone(),
                    });
            }
            WlSubcompositor::INTERFACE => {
                id.downcast::<WlSubcompositor>()
                    .set_handler(ClientWlSubcompositor);
            }
            WlSeat::INTERFACE => {
                id.downcast::<WlSeat>().set_handler(ClientWlSeat);
            }
            WlShm::INTERFACE => {
                id.downcast::<WlShm>().set_handler(ClientWlShm);
            }
            ZwpRelativePointerManagerV1::INTERFACE => {
                id.downcast::<ZwpRelativePointerManagerV1>()
                    .set_handler(ClientZwpRelativePointerManagerV1);
            }
            ZwpTextInputManagerV3::INTERFACE => {
                id.downcast::<ZwpTextInputManagerV3>()
                    .set_handler(ClientZwpTextInputManagerV3);
            }
            ZxdgDecorationManagerV1::INTERFACE => {
                id.set_forward_to_server(false);
                id.downcast::<ZxdgDecorationManagerV1>()
                    .set_handler(ClientZxdgDecorationManagerV1);
                return;
            }
            OrgKdeKwinServerDecorationManager::INTERFACE => {
                id.set_forward_to_server(false);
                id.downcast::<OrgKdeKwinServerDecorationManager>()
                    .set_handler(ClientOrgKdeKwinServerDecorationManager);
                return;
            }
            XdgToplevelIconManagerV1::INTERFACE => {
                id.set_forward_to_server(false);
                let id = id.downcast::<XdgToplevelIconManagerV1>();
                id.set_handler(ClientXdgToplevelIconManagerV1 {
                    globals: self.globals.clone(),
                });
                for size in [8, 16, 24, 32, 64] {
                    id.send_icon_size(size);
                }
                id.send_done();
                return;
            }
            _ => {}
        }
        self.mapper.handle_client_bind(slf, name, &id);
    }

    fn handle_global(
        &mut self,
        slf: &Rc<WlRegistry>,
        name: u32,
        interface: ObjectInterface,
        version: u32,
    ) {
        use ObjectInterface::*;
        if !self.init {
            self.init = true;
            let mut add_synth = |t, v| self.mapper.add_synthetic_global(slf, t, v);
            add_synth(XdgWmBase, 7);
            add_synth(ZxdgDecorationManagerV1, 1);
            add_synth(OrgKdeKwinServerDecorationManager, 1);
            add_synth(XdgToplevelIconManagerV1, 1);
        }
        match interface {
            ZxdgDecorationManagerV1
            | OrgKdeKwinServerDecorationManager
            | XdgWmBase
            | XdgWmDialogV1
            | XdgToplevelDragManagerV1
            | XdgToplevelIconManagerV1
            | WpSecurityContextManagerV1
            | XdgToplevelTagManagerV1 => {
                self.mapper.ignore_server_global(name);
            }
            _ => {
                self.mapper
                    .handle_server_global(slf, name, interface, version);
            }
        }
    }

    fn handle_global_remove(&mut self, slf: &Rc<WlRegistry>, name: u32) {
        self.mapper.handle_server_global_remove(slf, name);
    }
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
            subsurface_position: [0, 0],
            subsurfaces: Default::default(),
        });
        slf.send_create_surface(id);
    }

    fn handle_create_region(&mut self, slf: &Rc<WlCompositor>, id: &Rc<WlRegion>) {
        id.set_handler(ClientWlRegion {
            ops: Default::default(),
        });
        slf.send_create_region(id);
    }
}

impl WlSubcompositorHandler for ClientWlSubcompositor {
    fn handle_get_subsurface(
        &mut self,
        slf: &Rc<WlSubcompositor>,
        id: &Rc<WlSubsurface>,
        surface: &Rc<WlSurface>,
        parent: &Rc<WlSurface>,
    ) {
        let p = &mut *parent.get_handler_mut::<ClientWlSurface>();
        let c = &mut *surface.get_handler_mut::<ClientWlSurface>();
        p.subsurfaces.insert(surface.unique_id(), surface.clone());
        c.subsurface_position = [0, 0];
        c.set_input_mask(surface, p.input_mask);
        id.set_handler(ClientWlSubsurface {
            parent: parent.clone(),
            surface: surface.clone(),
        });
        slf.send_get_subsurface(id, surface, parent);
    }
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
        slf.send_destroy();
    }

    fn handle_set_position(&mut self, slf: &Rc<WlSubsurface>, dx: i32, dy: i32) {
        let p = &mut *self.parent.get_handler_mut::<ClientWlSurface>();
        let c = &mut *self.surface.get_handler_mut::<ClientWlSurface>();
        c.subsurface_position = [dx, dy];
        c.set_input_mask(&self.surface, p.input_mask);
        slf.send_set_position(dx, dy);
    }
}

impl ClientWlSurface {
    fn set_input_mask(&mut self, slf: &Rc<WlSurface>, parent_mask: Option<[i32; 4]>) {
        self.input_mask = parent_mask.map(|[x, y, w, h]| {
            [
                x - self.subsurface_position[0],
                y - self.subsurface_position[1],
                w,
                h,
            ]
        });
        self.update_region(slf);
        for surface in self.subsurfaces.values() {
            surface
                .get_handler_mut::<ClientWlSurface>()
                .set_input_mask(surface, self.input_mask);
        }
    }

    fn update_region(&self, slf: &Rc<WlSurface>) {
        let wl_region = self.globals.wl_compositor.new_send_create_region();
        let _destroy_region = on_drop(|| {
            wl_region.send_destroy();
        });
        let Some([x, y, w, h]) = self.input_mask else {
            let Some(region) = &self.client_input_region else {
                slf.send_set_input_region(None);
                return;
            };
            for op in region {
                match *op {
                    WlRegionOp::Add([x, y, w, h]) => {
                        wl_region.send_add(x, y, w, h);
                    }
                    WlRegionOp::Sub([x, y, w, h]) => {
                        wl_region.send_subtract(x, y, w, h);
                    }
                }
            }
            slf.send_set_input_region(Some(&wl_region));
            return;
        };
        let Some(region) = &self.client_input_region else {
            wl_region.send_add(x, y, w, h);
            slf.send_set_input_region(Some(&wl_region));
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
                    wl_region.send_add(x1, y1, x2 - x1, y2 - y1);
                }
                WlRegionOp::Sub([x, y, w, h]) => {
                    wl_region.send_subtract(x, y, w, h);
                }
            }
        }
        slf.send_set_input_region(Some(&wl_region));
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
        slf.send_attach(buffer, x, y);
    }

    fn handle_set_input_region(&mut self, slf: &Rc<WlSurface>, wl_region: Option<&Rc<WlRegion>>) {
        let region = wl_region.map(|r| r.get_handler_mut::<ClientWlRegion>().ops.clone());
        self.client_input_region = region;
        self.update_region(slf);
    }

    fn handle_commit(&mut self, slf: &Rc<WlSurface>) {
        let old_attached = self.attached;
        let new_attached = self.pending_attach.take().unwrap_or(old_attached);
        self.attached = new_attached;
        let Some(xdg_surface) = &self.xdg_surface else {
            slf.send_commit();
            return;
        };
        let h = &mut *xdg_surface.get_handler_mut::<ClientXdgSurface>();
        match (old_attached, new_attached) {
            (false, false) => {
                slf.send_commit();
                h.handle_initial_commit(xdg_surface);
            }
            (false, true) => {
                slf.send_commit();
                h.handle_map(xdg_surface);
            }
            (true, false) => {
                slf.send_commit();
                h.handle_unmap();
            }
            (true, true) => {
                let mut need_sync = false;
                if h.client_xdg_toplevel.is_some()
                    && let Some(geometry) = h.pending_geometry
                    && geometry != h.geometry
                {
                    need_sync = true;
                    h.geometry = geometry;
                    h.ensure_borders();
                }
                if need_sync {
                    h.subsurface.send_set_sync();
                }
                slf.send_commit();
                h.proxy_wl_surface.send_commit();
                if need_sync {
                    h.subsurface.send_set_desync();
                }
            }
        }
    }
}

impl ClientXdgSurface {
    fn handle_initial_commit(&mut self, xdg_surface: &Rc<XdgSurface>) {
        if self.proxy_xdg_popup.is_some() {
            self.proxy_wl_surface.send_commit();
        } else if let Some(client_xdg_toplevel) = &self.client_xdg_toplevel {
            let states = compute_toplevel_states(client_xdg_toplevel);
            let width = self.geometry[2];
            let height = self.geometry[3];
            client_xdg_toplevel.send_configure(width, height, uapi::as_bytes(&*states));
            let serial = self.create_client_serial(None);
            xdg_surface.send_configure(serial as u32);
        }
    }

    fn handle_map(&mut self, xdg_surface: &Rc<XdgSurface>) {
        if let Some(xdg_toplevel) = self.client_xdg_toplevel.clone() {
            let wl_display = self.wl_display.clone();
            let display = &mut *wl_display.get_handler_mut::<DisplayHandler>();
            for (&name, tray) in &display.trays {
                self.create_tray_item(name, tray, xdg_surface, &xdg_toplevel);
            }
        } else if self.proxy_xdg_popup.is_some() {
            self.proxy_wl_surface
                .send_attach(Some(&self.globals.transparent_spb), 0, 0);
            self.proxy_wl_surface.send_commit();
        }
    }

    fn handle_unmap(&mut self) {
        self.destroy_popups_recursive();
        if self.client_xdg_toplevel.is_some() {
            for (_, jay_tray_item_v1) in mem::take(&mut self.jay_tray_items) {
                jay_tray_item_v1
                    .get_handler_mut::<ProxyJayTrayItemV1>()
                    .destroy(&jay_tray_item_v1, self);
            }
        } else if let Some(xdg_popup) = &self.client_xdg_popup {
            if let Some(xdg_popup) = self.proxy_xdg_popup.take() {
                xdg_popup.send_destroy();
                xdg_popup.unset_handler();
            }
            xdg_popup.send_popup_done();
        }
        self.proxy_wl_surface.send_attach(None, 0, 0);
        self.proxy_wl_surface.send_commit();
    }

    fn destroy_popups_recursive(&mut self) {
        for (_, xdg_popup) in self.client_popups.drain() {
            let client_xdg_popup = &mut *xdg_popup.get_handler_mut::<ClientXdgPopup>();
            let client_xdg_surface = &mut *client_xdg_popup
                .xdg_surface
                .get_handler_mut::<ClientXdgSurface>();
            client_xdg_surface.destroy_popups_recursive();
            if let Some(xdg_popup) = client_xdg_surface.proxy_xdg_popup.take() {
                xdg_popup.send_destroy();
                xdg_popup.unset_handler();
            }
            xdg_popup.send_popup_done();
        }
    }

    fn create_tray_item(
        &mut self,
        name: u32,
        tray: &Rc<JayTrayV1>,
        xdg_surface: &Rc<XdgSurface>,
        xdg_toplevel: &Rc<XdgToplevel>,
    ) {
        let globals = &self.globals;
        let wl_surface = globals.wl_compositor.new_send_create_surface();
        wl_surface.set_forward_to_client(false);
        let wp_viewport = globals.wp_viewporter.new_send_get_viewport(&wl_surface);
        let jay_tray_item_v1 = tray.new_send_get_tray_item(&wl_surface);
        jay_tray_item_v1.set_forward_to_client(false);
        wl_surface.send_attach(Some(&globals.black_spb), 0, 0);
        wl_surface.send_commit();
        wl_surface.set_handler(ProxyTrayIconWlSurface {
            jay_tray_item_v1: jay_tray_item_v1.clone(),
        });
        let current = JayTrayItemV1Config {
            size: [1, 1],
            preferred_anchor: XdgPositionerAnchor::BOTTOM_RIGHT,
            preferred_gravity: XdgPositionerGravity::BOTTOM_LEFT,
        };
        jay_tray_item_v1.set_handler(ProxyJayTrayItemV1 {
            globals: self.globals.clone(),
            client_xdg_surface: xdg_surface.clone(),
            client_xdg_toplevel: xdg_toplevel.clone(),
            proxy_xdg_popup: Default::default(),
            wl_surface,
            wp_viewport,
            pending: current,
            current,
        });
        self.jay_tray_items.insert(name, jay_tray_item_v1);
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

    fn ensure_borders(&mut self) {
        let borders = self.tray_popup_borders.as_mut().unwrap();
        for (k, v) in borders {
            if self.needed_tray_edges & (1 << k.linearize()) == 0 {
                v.wl_surface.send_attach(None, 0, 0);
                v.wl_surface.send_commit();
                v.buffer_size = [0, 0];
                continue;
            }
            // todo
            let length = 20;
            let length32 = length as i32;
            {
                let (x, y) = match k {
                    WindowEdge::Top => (0, -length32),
                    WindowEdge::TopRight => (self.geometry[2], -length32),
                    WindowEdge::Right => (self.geometry[2], 0),
                    WindowEdge::BottomRight => (self.geometry[2], self.geometry[3]),
                    WindowEdge::Bottom => (0, self.geometry[3]),
                    WindowEdge::BottomLeft => (-length32, self.geometry[3]),
                    WindowEdge::Left => (-length32, 0),
                    WindowEdge::TopLeft => (-length32, -length32),
                };
                v.wl_subsurface
                    .send_set_position(x + self.geometry[0], y + self.geometry[1]);
            }
            let required_size = match k {
                WindowEdge::Top | WindowEdge::Bottom => [self.geometry[2] as usize, length],
                WindowEdge::Right | WindowEdge::Left => [self.geometry[3] as usize, length],
                WindowEdge::TopRight
                | WindowEdge::BottomRight
                | WindowEdge::BottomLeft
                | WindowEdge::TopLeft => [length, length],
            };
            if v.buffer_size == required_size {
                continue;
            }
            let mut mem = vec![0u32; required_size[0] * required_size[1]];
            if matches!(
                k,
                WindowEdge::TopRight
                    | WindowEdge::BottomRight
                    | WindowEdge::BottomLeft
                    | WindowEdge::TopLeft
            ) {
                let lengthf = length as f32;
                for x in 0..length {
                    for y in 0..length {
                        let dist = match k {
                            WindowEdge::TopRight => {
                                ((x.pow(2) + (length - y - 1).pow(2)) as f32).sqrt() / lengthf
                            }
                            WindowEdge::BottomRight => {
                                ((x.pow(2) + y.pow(2)) as f32).sqrt() / lengthf
                            }
                            WindowEdge::BottomLeft => {
                                (((length - x - 1).pow(2) + y.pow(2)) as f32).sqrt() / lengthf
                            }
                            WindowEdge::TopLeft => {
                                (((length - x - 1).pow(2) + (length - y - 1).pow(2)) as f32).sqrt()
                                    / lengthf
                            }
                            _ => unreachable!(),
                        };
                        let alpha = 0.5 * (1.0 - dist);
                        let v = u32::from_le_bytes([0, 0, 0, (alpha * 255.0) as u8]);
                        mem[y * length + x] = v;
                    }
                }
            } else {
                for y in 0..length {
                    let dist = (length - y - 1) as f32 / length as f32;
                    let alpha = 0.5 * (1.0 - dist);
                    let v = u32::from_le_bytes([0, 0, 0, (alpha * 255.0) as u8]);
                    mem[y * required_size[0]..][..required_size[0]].fill(v);
                }
            }
            let mut fd = uapi::memfd_create("", c::MFD_CLOEXEC | c::MFD_ALLOW_SEALING).unwrap();
            uapi::fcntl_add_seals(fd.raw(), c::F_SEAL_SHRINK).unwrap();
            fd.write_all(uapi::as_bytes(&*mem)).unwrap();
            let wl_shm_pool = self
                .globals
                .wl_shm
                .new_send_create_pool(&Rc::new(fd.into()), size_of_val(&*mem) as _);
            let wl_buffer = wl_shm_pool.new_send_create_buffer(
                0,
                required_size[0] as i32,
                required_size[1] as i32,
                required_size[0] as i32 * 4,
                WlShmFormat::ARGB8888,
            );
            v.wl_surface.send_attach(Some(&wl_buffer), 0, 0);
            v.wl_surface.send_commit();
            wl_buffer.send_destroy();
            wl_shm_pool.send_destroy();
            v.buffer_size = required_size;
        }
    }
}

impl XdgWmBaseHandler for ClientXdgWmBase {
    fn handle_destroy(&mut self, slf: &Rc<XdgWmBase>) {
        slf.unset_handler();
        slf.delete_id();
    }

    fn handle_create_positioner(&mut self, _slf: &Rc<XdgWmBase>, id: &Rc<XdgPositioner>) {
        self.globals.xdg_wm_base.send_create_positioner(id);
    }

    fn handle_get_xdg_surface(
        &mut self,
        _slf: &Rc<XdgWmBase>,
        client_xdg_surface: &Rc<XdgSurface>,
        client_surface: &Rc<WlSurface>,
    ) {
        let g = &*self.globals;
        let proxy_surface = g.wl_compositor.new_send_create_surface();
        proxy_surface.set_forward_to_client(false);
        proxy_surface.send_set_input_region(Some(&g.empty_region));
        let subsurface = g
            .wl_subcompositor
            .new_send_get_subsurface(client_surface, &proxy_surface);
        subsurface.send_set_desync();
        proxy_surface.send_commit();
        let proxy_xdg_surface = g.xdg_wm_base.new_send_get_xdg_surface(&proxy_surface);
        proxy_xdg_surface.set_forward_to_client(false);
        proxy_xdg_surface.set_handler(ProxyXdgSurface {
            client_xdg_surface: client_xdg_surface.clone(),
        });
        proxy_surface.set_handler(ProxyXdgSurfaceWlSurface {
            client_wl_surface: client_surface.clone(),
        });
        client_xdg_surface.set_handler(ClientXdgSurface {
            wl_display: self.wl_display.clone(),
            globals: self.globals.clone(),
            client_wl_surface: client_surface.clone(),
            client_xdg_popup: Default::default(),
            client_xdg_toplevel: Default::default(),
            proxy_wl_surface: proxy_surface,
            proxy_xdg_surface,
            proxy_xdg_popup: Default::default(),
            subsurface,
            popup_jay_tray_item: Default::default(),
            jay_tray_items: Default::default(),
            client_popups: Default::default(),
            geometry: [0, 0, 800, 600],
            pending_geometry: None,
            map_unpon_configure: Default::default(),
            map_unpon_ack: None,
            next_client_serial: 1,
            pending_serials: Default::default(),
            needed_tray_edges: 0,
            tray_popup_borders: Default::default(),
            toplevel_icon: Default::default(),
            last_configure_size: None,
            ignore_configure: false,
        });
        let client_surface_handler = &mut *client_surface.get_handler_mut::<ClientWlSurface>();
        client_surface_handler.xdg_surface = Some(client_xdg_surface.clone());
    }
}

impl WlSurfaceHandler for ProxyXdgSurfaceWlSurface {}

impl XdgSurfaceHandler for ClientXdgSurface {
    fn handle_destroy(&mut self, slf: &Rc<XdgSurface>) {
        self.client_wl_surface
            .get_handler_mut::<ClientWlSurface>()
            .xdg_surface = None;
        self.proxy_xdg_surface.send_destroy();
        self.subsurface.send_destroy();
        self.proxy_wl_surface.unset_handler();
        self.proxy_wl_surface.send_destroy();
        slf.unset_handler();
        slf.delete_id();
    }

    fn handle_get_toplevel(&mut self, slf: &Rc<XdgSurface>, id: &Rc<XdgToplevel>) {
        id.set_forward_to_server(false);
        let display = &mut *self.wl_display.get_handler_mut::<DisplayHandler>();
        display.toplevels.insert(id.unique_id(), slf.clone());
        id.set_handler(ClientXdgToplevel {
            xdg_surface: slf.clone(),
        });
        let borders = StaticMap::from_fn(|e| {
            let wl_surface = self.globals.wl_compositor.new_send_create_surface();
            wl_surface.set_forward_to_client(false);
            wl_surface.set_handler(ProxyTrayPopupBorderWlSurface {
                client_xdg_surface: slf.clone(),
                window_edge: e,
            });
            let rotation = match e {
                WindowEdge::Top => WlOutputTransform::NORMAL,
                WindowEdge::Right => WlOutputTransform::_90,
                WindowEdge::Bottom => WlOutputTransform::_180,
                WindowEdge::Left => WlOutputTransform::_270,
                WindowEdge::TopRight
                | WindowEdge::BottomRight
                | WindowEdge::BottomLeft
                | WindowEdge::TopLeft => WlOutputTransform::NORMAL,
            };
            wl_surface.send_set_buffer_transform(rotation);
            let wl_subsurface = self
                .globals
                .wl_subcompositor
                .new_send_get_subsurface(&wl_surface, &self.proxy_wl_surface);
            TrayPopupBorder {
                wl_surface,
                wl_subsurface,
                buffer_size: [0, 0],
            }
        });
        self.tray_popup_borders = Some(borders);
        self.client_xdg_toplevel = Some(id.clone());
        if id.version() >= XdgToplevel::MSG__WM_CAPABILITIES__SINCE {
            id.send_wm_capabilities(&[]);
        }
    }

    fn handle_get_popup(
        &mut self,
        slf: &Rc<XdgSurface>,
        id: &Rc<XdgPopup>,
        parent: Option<&Rc<XdgSurface>>,
        positioner: &Rc<XdgPositioner>,
    ) {
        id.set_handler(ClientXdgPopup {
            xdg_surface: slf.clone(),
            parent_xdg_surface: parent.cloned(),
        });
        let mut parent_xdg_surface = parent.map(|p| p.get_handler_mut::<ClientXdgSurface>());
        if let Some(parent_xdg_surface) = &mut parent_xdg_surface {
            parent_xdg_surface
                .client_popups
                .insert(id.unique_id(), id.clone());
        }
        let parent = parent_xdg_surface.as_ref().map(|p| &p.proxy_xdg_surface);
        self.client_xdg_popup = Some(id.clone());
        let proxy_xdg_popup = self
            .proxy_xdg_surface
            .new_send_get_popup(parent, positioner);
        proxy_xdg_popup.set_forward_to_client(false);
        proxy_xdg_popup.set_handler(ProxyClientPopupXdgPopup {
            xdg_surface: slf.clone(),
            client_xdg_popup: id.clone(),
        });
        self.proxy_xdg_popup = Some(proxy_xdg_popup);
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
        self.proxy_xdg_surface
            .send_set_window_geometry(x, y, width, height);
        self.pending_geometry = Some([x, y, width, height]);
    }

    fn handle_ack_configure(&mut self, _slf: &Rc<XdgSurface>, client_serial: u32) {
        let client_serial = self.recover_client_serial(client_serial);
        while let Some(&(expected, server_serial)) = self.pending_serials.front() {
            if expected <= client_serial {
                if let Some(server_serial) = server_serial {
                    self.proxy_xdg_surface.send_ack_configure(server_serial);
                }
                self.pending_serials.pop_front();
            } else {
                break;
            }
        }
        if let Some(serial) = self.map_unpon_ack
            && serial <= client_serial
        {
            self.map_unpon_ack = None;
            self.ensure_borders();
            self.proxy_wl_surface
                .send_attach(Some(&self.globals.transparent_spb), 0, 0);
            self.proxy_wl_surface.send_commit();
        }
    }
}

impl XdgPopupHandler for ProxyClientPopupXdgPopup {
    fn handle_configure(&mut self, _slf: &Rc<XdgPopup>, x: i32, y: i32, width: i32, height: i32) {
        self.client_xdg_popup.send_configure(x, y, width, height);
    }

    fn handle_popup_done(&mut self, _slf: &Rc<XdgPopup>) {
        self.xdg_surface
            .get_handler_mut::<ClientXdgSurface>()
            .handle_unmap();
    }

    fn handle_repositioned(&mut self, _slf: &Rc<XdgPopup>, token: u32) {
        self.client_xdg_popup.send_repositioned(token);
    }
}

impl ClientXdgPopup {
    fn get_handler(&self) -> HandlerMut<'_, ClientXdgSurface> {
        self.xdg_surface.get_handler_mut()
    }
}

impl WlSurfaceHandler for ProxyTrayPopupBorderWlSurface {}

impl XdgToplevelHandler for ClientXdgToplevel {
    fn handle_destroy(&mut self, slf: &Rc<XdgToplevel>) {
        let h = &mut *self.xdg_surface.get_handler_mut::<ClientXdgSurface>();
        h.handle_unmap();
        h.client_xdg_toplevel = None;
        if let Some(edges) = h.tray_popup_borders.take() {
            for edge in edges.values() {
                edge.wl_surface.unset_handler();
                edge.wl_subsurface.send_destroy();
                edge.wl_surface.send_destroy();
            }
        }
        h.wl_display
            .get_handler_mut::<DisplayHandler>()
            .toplevels
            .remove(&slf.unique_id());
        slf.unset_handler();
        slf.delete_id();
    }
}

impl XdgPopupHandler for ClientXdgPopup {
    fn handle_destroy(&mut self, slf: &Rc<XdgPopup>) {
        let h = &mut *self.get_handler();
        h.handle_unmap();
        h.client_xdg_popup = None;
        slf.unset_handler();
        slf.delete_id();
        if let Some(parent) = &self.parent_xdg_surface {
            parent
                .get_handler_mut::<ClientXdgSurface>()
                .client_popups
                .remove(&slf.unique_id());
        }
    }

    fn handle_grab(&mut self, _slf: &Rc<XdgPopup>, seat: &Rc<WlSeat>, serial: u32) {
        let h = self.get_handler();
        if let Some(id) = &h.proxy_xdg_popup {
            id.send_grab(seat, serial);
        }
    }

    fn handle_reposition(
        &mut self,
        _slf: &Rc<XdgPopup>,
        positioner: &Rc<XdgPositioner>,
        token: u32,
    ) {
        let h = self.get_handler();
        if let Some(id) = &h.proxy_xdg_popup {
            id.send_reposition(positioner, token);
        }
    }
}

impl ProxyWlSeat {
    fn handle_capabilities(&mut self, capabilities: WlSeatCapability) {
        if capabilities.contains(WlSeatCapability::POINTER) {
            if self.wl_pointer.is_none() {
                let proxy = self.wl_seat.new_send_get_pointer();
                proxy.set_forward_to_client(false);
                let wp_cursor_shape_device_v1 = self
                    .globals
                    .wp_cursor_shape_manager_v1
                    .new_send_get_pointer(&proxy);
                proxy.set_handler(ProxyWlPointer {
                    globals: self.globals.clone(),
                    seat: self.wl_seat.clone(),
                    wp_cursor_shape_device_v1,
                    tray_icon_focus: None,
                    edge_focus: None,
                    surface: None,
                });
                self.wl_pointer = Some(proxy);
            }
        } else {
            if let Some(proxy) = self.wl_pointer.take() {
                let h = &mut *proxy.get_handler_mut::<ProxyWlPointer>();
                h.tray_icon_focus = None;
                h.wp_cursor_shape_device_v1.unset_handler();
                h.wp_cursor_shape_device_v1.send_destroy();
                proxy.unset_handler();
                proxy.send_release();
            }
        }
    }
}

impl WlSeatHandler for ProxyWlSeat {
    fn handle_capabilities(&mut self, _slf: &Rc<WlSeat>, capabilities: WlSeatCapability) {
        self.handle_capabilities(capabilities);
    }
}

impl WlRegionHandler for ClientWlRegion {
    fn handle_add(&mut self, slf: &Rc<WlRegion>, x: i32, y: i32, width: i32, height: i32) {
        self.ops.push(WlRegionOp::Add([x, y, width, height]));
        slf.send_add(x, y, width, height);
    }

    fn handle_subtract(&mut self, slf: &Rc<WlRegion>, x: i32, y: i32, width: i32, height: i32) {
        self.ops.push(WlRegionOp::Sub([x, y, width, height]));
        slf.send_subtract(x, y, width, height);
    }
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
        self.surface = Some(surface.clone());
        if let Ok(tsh) = surface.try_get_handler_mut::<ProxyTrayIconWlSurface>() {
            self.wp_cursor_shape_device_v1
                .send_set_shape(serial, WpCursorShapeDeviceV1Shape::DEFAULT);
            self.tray_icon_focus = Some(tsh.jay_tray_item_v1.clone());
        } else if let Ok(tsh) = surface.try_get_handler_mut::<ProxyTrayPopupBorderWlSurface>() {
            let shape = match tsh.window_edge {
                WindowEdge::Top | WindowEdge::Bottom => WpCursorShapeDeviceV1Shape::NS_RESIZE,
                WindowEdge::Right | WindowEdge::Left => WpCursorShapeDeviceV1Shape::EW_RESIZE,
                WindowEdge::TopRight | WindowEdge::BottomLeft => {
                    WpCursorShapeDeviceV1Shape::NESW_RESIZE
                }
                WindowEdge::BottomRight | WindowEdge::TopLeft => {
                    WpCursorShapeDeviceV1Shape::NWSE_RESIZE
                }
            };
            self.wp_cursor_shape_device_v1.send_set_shape(serial, shape);
            self.edge_focus = Some(tsh.window_edge);
        }
    }

    fn handle_leave(&mut self, _slf: &Rc<WlPointer>, _serial: u32, _surface: &Rc<WlSurface>) {
        self.surface = None;
        self.tray_icon_focus = None;
        self.edge_focus = None;
    }

    fn handle_button(
        &mut self,
        _slf: &Rc<WlPointer>,
        serial: u32,
        _time: u32,
        button: u32,
        state: WlPointerButtonState,
    ) {
        const BTN_LEFT: u32 = 0x110;
        const BTN_MIDDLE: u32 = 0x112;
        if state != WlPointerButtonState::PRESSED {
            return;
        }
        if button == BTN_LEFT
            && let Some(surface) = &self.surface
            && let Ok(proxy_tray_popup_border_wl_surface) =
                surface.try_get_handler_mut::<ProxyTrayPopupBorderWlSurface>()
            && let Some(jay_tray_item_v1) = &proxy_tray_popup_border_wl_surface
                .client_xdg_surface
                .get_handler_mut::<ClientXdgSurface>()
                .popup_jay_tray_item
            && let Some(xdg_popup) = &jay_tray_item_v1
                .get_handler_mut::<ProxyJayTrayItemV1>()
                .proxy_xdg_popup
        {
            xdg_popup
                .get_handler_mut::<ProxyTrayXdgPopup>()
                .jay_popup_ext_v1
                .send_resize(
                    &self.seat,
                    serial,
                    proxy_tray_popup_border_wl_surface.window_edge.to_wl(),
                );
            return;
        }
        let Some(pf) = &self.tray_icon_focus else {
            return;
        };
        let tray_item_handler = &mut *pf.get_handler_mut::<ProxyJayTrayItemV1>();
        if button == BTN_MIDDLE {
            tray_item_handler.client_xdg_toplevel.send_close();
            return;
        }
        let client_xdg_surface = tray_item_handler.client_xdg_surface.clone();
        let xdg_surface_handler = &mut *client_xdg_surface.get_handler_mut::<ClientXdgSurface>();
        if let Some(popup) = tray_item_handler.proxy_xdg_popup.take() {
            popup.get_handler_mut::<ProxyTrayXdgPopup>().destroy(
                &popup,
                tray_item_handler,
                xdg_surface_handler,
            );
            return;
        }
        if let Some(other) = xdg_surface_handler.popup_jay_tray_item.clone() {
            let other = &mut other.get_handler_mut::<ProxyJayTrayItemV1>();
            if let Some(popup) = other.proxy_xdg_popup.take() {
                popup.get_handler_mut::<ProxyTrayXdgPopup>().destroy(
                    &popup,
                    other,
                    xdg_surface_handler,
                );
            }
        }
        xdg_surface_handler.popup_jay_tray_item = Some(pf.clone());
        let xdg_positioner = tray_item_handler.create_positioner(xdg_surface_handler);
        let xdg_popup = xdg_surface_handler
            .proxy_xdg_surface
            .new_send_get_popup(None, &xdg_positioner);
        xdg_popup.set_forward_to_client(false);
        xdg_positioner.send_destroy();
        pf.send_get_popup(
            &xdg_popup,
            &self.seat,
            serial,
            JayTrayItemV1KeyboardFocusHint::IMMEDIATE,
        );
        let jay_popup_ext_v1 = self
            .globals
            .jay_popup_ext_manager_v1
            .new_send_get_ext(&xdg_popup);
        xdg_surface_handler.proxy_wl_surface.send_commit();
        xdg_popup.set_handler(ProxyTrayXdgPopup {
            jay_tray_item_v1: pf.clone(),
            jay_popup_ext_v1,
        });
        tray_item_handler.proxy_xdg_popup = Some(xdg_popup);
        xdg_surface_handler.map_unpon_configure = true;
    }
}

impl WlSeatHandler for ClientWlSeat {
    fn handle_get_pointer(&mut self, slf: &Rc<WlSeat>, id: &Rc<WlPointer>) {
        id.set_handler(ClientWlSeatDevice::default());
        slf.send_get_pointer(id);
    }

    fn handle_get_keyboard(&mut self, slf: &Rc<WlSeat>, id: &Rc<WlKeyboard>) {
        id.set_handler(ClientWlSeatDevice::default());
        slf.send_get_keyboard(id);
    }

    fn handle_get_touch(&mut self, slf: &Rc<WlSeat>, id: &Rc<WlTouch>) {
        id.set_handler(ClientWlSeatDevice::default());
        slf.send_get_touch(id);
    }
}

impl WlPointerHandler for ClientWlSeatDevice {
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
            slf.send_enter(serial, surface, surface_x, surface_y);
        }
    }

    fn handle_leave(&mut self, slf: &Rc<WlPointer>, serial: u32, surface: &Rc<WlSurface>) {
        if self.on_client_surface {
            slf.send_leave(serial, surface);
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
            slf.send_motion(time, surface_x, surface_y);
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
            slf.send_button(serial, time, button, state);
        }
    }

    fn handle_axis(&mut self, slf: &Rc<WlPointer>, time: u32, axis: WlPointerAxis, value: Fixed) {
        if self.on_client_surface {
            slf.send_axis(time, axis, value);
        }
    }

    fn handle_frame(&mut self, slf: &Rc<WlPointer>) {
        slf.send_frame();
    }

    fn handle_axis_source(&mut self, slf: &Rc<WlPointer>, axis_source: WlPointerAxisSource) {
        if self.on_client_surface {
            slf.send_axis_source(axis_source);
        }
    }

    fn handle_axis_stop(&mut self, slf: &Rc<WlPointer>, time: u32, axis: WlPointerAxis) {
        if self.on_client_surface {
            slf.send_axis_stop(time, axis);
        }
    }

    fn handle_axis_discrete(&mut self, slf: &Rc<WlPointer>, axis: WlPointerAxis, discrete: i32) {
        if self.on_client_surface {
            slf.send_axis_discrete(axis, discrete);
        }
    }

    fn handle_axis_value120(&mut self, slf: &Rc<WlPointer>, axis: WlPointerAxis, value120: i32) {
        if self.on_client_surface {
            slf.send_axis_value120(axis, value120);
        }
    }

    fn handle_axis_relative_direction(
        &mut self,
        slf: &Rc<WlPointer>,
        axis: WlPointerAxis,
        direction: WlPointerAxisRelativeDirection,
    ) {
        if self.on_client_surface {
            slf.send_axis_relative_direction(axis, direction);
        }
    }
}

impl WlTouchHandler for ClientWlSeatDevice {
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
            slf.send_down(serial, time, surface, id, x, y);
        }
    }

    fn handle_up(&mut self, slf: &Rc<WlTouch>, serial: u32, time: u32, id: i32) {
        if self.on_client_surface {
            slf.send_up(serial, time, id);
        }
    }

    fn handle_motion(&mut self, slf: &Rc<WlTouch>, time: u32, id: i32, x: Fixed, y: Fixed) {
        if self.on_client_surface {
            slf.send_motion(time, id, x, y);
        }
    }

    fn handle_frame(&mut self, slf: &Rc<WlTouch>) {
        slf.send_frame();
    }

    fn handle_cancel(&mut self, slf: &Rc<WlTouch>) {
        if self.on_client_surface {
            slf.send_cancel();
        }
    }

    fn handle_shape(&mut self, slf: &Rc<WlTouch>, id: i32, major: Fixed, minor: Fixed) {
        if self.on_client_surface {
            slf.send_shape(id, major, minor);
        }
    }

    fn handle_orientation(&mut self, slf: &Rc<WlTouch>, id: i32, orientation: Fixed) {
        if self.on_client_surface {
            slf.send_orientation(id, orientation);
        }
    }
}

impl WlKeyboardHandler for ClientWlSeatDevice {
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
        slf.send_enter(serial, &surface_handler.client_wl_surface, keys);
    }

    fn handle_leave(&mut self, slf: &Rc<WlKeyboard>, serial: u32, surface: &Rc<WlSurface>) {
        if self.on_client_surface {
            let Ok(surface_handler) = surface.try_get_handler_mut::<ProxyXdgSurfaceWlSurface>()
            else {
                return;
            };
            slf.send_leave(serial, &surface_handler.client_wl_surface);
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
            slf.send_key(serial, time, key, state);
        }
    }
}

impl ZwpTextInputManagerV3Handler for ClientZwpTextInputManagerV3 {
    fn handle_get_text_input(
        &mut self,
        slf: &Rc<ZwpTextInputManagerV3>,
        id: &Rc<ZwpTextInputV3>,
        seat: &Rc<WlSeat>,
    ) {
        id.set_handler(ClientWlSeatDevice::default());
        slf.send_get_text_input(id, seat);
    }
}

impl ZwpTextInputV3Handler for ClientWlSeatDevice {
    fn handle_enter(&mut self, slf: &Rc<ZwpTextInputV3>, surface: &Rc<WlSurface>) {
        self.on_client_surface = surface.try_get_handler_mut::<ClientWlSurface>().is_ok();
        if self.on_client_surface {
            slf.send_enter(surface);
        }
    }

    fn handle_leave(&mut self, slf: &Rc<ZwpTextInputV3>, surface: &Rc<WlSurface>) {
        if self.on_client_surface {
            slf.send_leave(surface);
        }
    }

    fn handle_preedit_string(
        &mut self,
        slf: &Rc<ZwpTextInputV3>,
        text: Option<&str>,
        cursor_begin: i32,
        cursor_end: i32,
    ) {
        if self.on_client_surface {
            slf.send_preedit_string(text, cursor_begin, cursor_end);
        }
    }

    fn handle_commit_string(&mut self, slf: &Rc<ZwpTextInputV3>, text: Option<&str>) {
        if self.on_client_surface {
            slf.send_commit_string(text);
        }
    }

    fn handle_delete_surrounding_text(
        &mut self,
        slf: &Rc<ZwpTextInputV3>,
        before_length: u32,
        after_length: u32,
    ) {
        if self.on_client_surface {
            slf.send_delete_surrounding_text(before_length, after_length);
        }
    }
}

impl ZwpRelativePointerManagerV1Handler for ClientZwpRelativePointerManagerV1 {
    fn handle_get_relative_pointer(
        &mut self,
        slf: &Rc<ZwpRelativePointerManagerV1>,
        id: &Rc<ZwpRelativePointerV1>,
        pointer: &Rc<WlPointer>,
    ) {
        id.set_handler(ClientZwpRelativePointerV1 {
            wl_pointer: pointer.clone(),
        });
        slf.send_get_relative_pointer(id, pointer);
    }
}

impl ZwpRelativePointerV1Handler for ClientZwpRelativePointerV1 {
    fn handle_relative_motion(
        &mut self,
        slf: &Rc<ZwpRelativePointerV1>,
        utime_hi: u32,
        utime_lo: u32,
        dx: Fixed,
        dy: Fixed,
        dx_unaccel: Fixed,
        dy_unaccel: Fixed,
    ) {
        if self
            .wl_pointer
            .get_handler_mut::<ClientWlSeatDevice>()
            .on_client_surface
        {
            slf.send_relative_motion(utime_hi, utime_lo, dx, dy, dx_unaccel, dy_unaccel);
        }
    }
}

impl XdgSurfaceHandler for ProxyXdgSurface {
    fn handle_configure(&mut self, slf: &Rc<XdgSurface>, server_serial: u32) {
        let h = &mut *self
            .client_xdg_surface
            .get_handler_mut::<ClientXdgSurface>();
        let client_serial;
        if h.ignore_configure {
            if h.pending_serials.is_empty() {
                slf.send_ack_configure(server_serial);
                if h.map_unpon_configure {
                    h.map_unpon_configure = false;
                    h.ensure_borders();
                    h.proxy_wl_surface
                        .send_attach(Some(&h.globals.transparent_spb), 0, 0);
                    h.proxy_wl_surface.send_commit();
                }
                return;
            } else {
                client_serial = h.next_client_serial - 1;
                h.pending_serials
                    .push_back((client_serial, Some(server_serial)));
            }
        } else {
            client_serial = h.create_client_serial(Some(server_serial));
            self.client_xdg_surface.send_configure(client_serial as u32);
        }
        if h.map_unpon_configure {
            h.map_unpon_configure = false;
            h.map_unpon_ack = Some(client_serial);
        }
    }
}

impl ProxyTrayXdgPopup {
    fn destroy(
        &self,
        popup: &Rc<XdgPopup>,
        item: &mut ProxyJayTrayItemV1,
        client_xdg_surface: &mut ClientXdgSurface,
    ) {
        self.jay_popup_ext_v1.send_destroy();
        item.proxy_xdg_popup = None;
        client_xdg_surface.destroy_popups_recursive();
        popup.unset_handler();
        popup.send_destroy();
        client_xdg_surface.popup_jay_tray_item = None;
        client_xdg_surface.proxy_wl_surface.send_attach(None, 0, 0);
        client_xdg_surface.proxy_wl_surface.send_commit();
    }
}

impl XdgPopupHandler for ProxyTrayXdgPopup {
    fn handle_configure(&mut self, _slf: &Rc<XdgPopup>, _x: i32, _y: i32, width: i32, height: i32) {
        let tray_item_handler = self
            .jay_tray_item_v1
            .get_handler_mut::<ProxyJayTrayItemV1>();
        let client_xdg_surface = &mut *tray_item_handler
            .client_xdg_surface
            .get_handler_mut::<ClientXdgSurface>();
        if client_xdg_surface.last_configure_size == Some([width, height]) {
            client_xdg_surface.ignore_configure = true;
            return;
        }
        client_xdg_surface.ignore_configure = false;
        client_xdg_surface.last_configure_size = Some([width, height]);
        let states = compute_toplevel_states(&tray_item_handler.client_xdg_toplevel);
        tray_item_handler.client_xdg_toplevel.send_configure(
            width,
            height,
            uapi::as_bytes(&*states),
        );
    }

    fn handle_popup_done(&mut self, slf: &Rc<XdgPopup>) {
        let tray_item_handler = &mut *self
            .jay_tray_item_v1
            .get_handler_mut::<ProxyJayTrayItemV1>();
        let client_xdg_surface = tray_item_handler.client_xdg_surface.clone();
        let client_xdg_surface = &mut *client_xdg_surface.get_handler_mut::<ClientXdgSurface>();
        self.destroy(slf, tray_item_handler, client_xdg_surface);
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

impl WlSurfaceHandler for ProxyTrayIconWlSurface {}

impl ProxyJayTrayItemV1 {
    fn destroy(&mut self, slf: &Rc<JayTrayItemV1>, client_xdg_surface: &mut ClientXdgSurface) {
        if let Some(popup) = self.proxy_xdg_popup.take() {
            popup
                .get_handler_mut::<ProxyTrayXdgPopup>()
                .destroy(&popup, self, client_xdg_surface);
        }
        slf.unset_handler();
        self.wl_surface.unset_handler();
        slf.send_destroy();
        self.wp_viewport.send_destroy();
        self.wl_surface.send_destroy();
    }

    fn create_positioner(&self, client_xdg_surface: &ClientXdgSurface) -> Rc<XdgPositioner> {
        let globals = &client_xdg_surface.globals;
        let config = self.current;
        let xdg_positioner = globals.xdg_wm_base.new_send_create_positioner();
        xdg_positioner.send_set_reactive();
        xdg_positioner.send_set_anchor_rect(0, 0, config.size[0], config.size[1]);
        xdg_positioner.send_set_anchor(config.preferred_anchor);
        xdg_positioner.send_set_gravity(config.preferred_gravity);
        xdg_positioner.send_set_size(
            client_xdg_surface.geometry[2],
            client_xdg_surface.geometry[3],
        );
        xdg_positioner.send_set_constraint_adjustment(
            XdgPositionerConstraintAdjustment::RESIZE_X
                | XdgPositionerConstraintAdjustment::RESIZE_Y,
        );
        xdg_positioner
    }

    fn update_icon(&mut self, client_xdg_surface: &ClientXdgSurface, commit: bool) {
        self.wl_surface.send_damage(0, 0, i32::MAX, i32::MAX);
        let Some(icons) = &client_xdg_surface.toplevel_icon else {
            self.wl_surface
                .send_attach(Some(&self.globals.black_spb), 0, 0);
            if commit {
                self.wl_surface.send_commit();
            }
            return;
        };
        let mut icon = &icons[0];
        for cand in &icons[1..] {
            if icon.logical_size < self.current.size[0] {
                if (cand.logical_size, cand.buffer_size) > (icon.logical_size, icon.buffer_size) {
                    icon = cand;
                }
            } else if cand.logical_size >= self.current.size[0] {
                if (cand.logical_size, cand.buffer_size) < (icon.logical_size, icon.buffer_size) {
                    icon = cand;
                }
            }
        }
        self.wl_surface.send_attach(Some(&icon.buffer), 0, 0);
        if commit {
            self.wl_surface.send_commit();
        }
    }
}

impl JayTrayItemV1Handler for ProxyJayTrayItemV1 {
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
        let client_xdg_surface = self.client_xdg_surface.clone();
        let client_xdg_surface = &mut *client_xdg_surface.get_handler_mut::<ClientXdgSurface>();
        slf.send_ack_configure(serial);
        if self.pending.size != self.current.size {
            self.wp_viewport
                .send_set_destination(self.pending.size[0], self.pending.size[1]);
        }
        self.current = self.pending;
        self.update_icon(client_xdg_surface, false);
        self.wl_surface.send_commit();
        let mut needed_tray_edges = 0;
        macro_rules! need {
            ($($edge:ident),*) => {{
                $(
                    needed_tray_edges |= 1 << WindowEdge::$edge.linearize();
                )*
            }};
        }
        match self.current.preferred_gravity {
            XdgPositionerGravity::NONE => {}
            XdgPositionerGravity::TOP => need!(Left, TopLeft, Top, TopRight, Right),
            XdgPositionerGravity::BOTTOM => need!(Left, BottomLeft, Bottom, BottomRight, Right),
            XdgPositionerGravity::LEFT => need!(Top, TopLeft, Left, BottomLeft, Bottom),
            XdgPositionerGravity::RIGHT => need!(Top, TopRight, Right, BottomRight, Bottom),
            XdgPositionerGravity::TOP_LEFT => need!(Top, TopLeft, Left),
            XdgPositionerGravity::BOTTOM_LEFT => need!(Bottom, BottomLeft, Left),
            XdgPositionerGravity::TOP_RIGHT => need!(Top, TopRight, Right),
            XdgPositionerGravity::BOTTOM_RIGHT => need!(Bottom, BottomRight, Right),
            _ => {}
        }
        client_xdg_surface.needed_tray_edges = needed_tray_edges;
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

impl XdgToplevelIconManagerV1Handler for ClientXdgToplevelIconManagerV1 {
    fn handle_destroy(&mut self, slf: &Rc<XdgToplevelIconManagerV1>) {
        slf.unset_handler();
        slf.delete_id();
    }

    fn handle_create_icon(
        &mut self,
        _slf: &Rc<XdgToplevelIconManagerV1>,
        id: &Rc<XdgToplevelIconV1>,
    ) {
        id.set_handler(ClientXdgToplevelIconV1 {
            buffers: Default::default(),
        });
    }

    fn handle_set_icon(
        &mut self,
        _slf: &Rc<XdgToplevelIconManagerV1>,
        toplevel: &Rc<XdgToplevel>,
        icon: Option<&Rc<XdgToplevelIconV1>>,
    ) {
        let mut icons = None;
        if let Some(icon) = icon {
            let mut vec = vec![];
            for (&(size, scale), buffer) in
                &icon.get_handler_mut::<ClientXdgToplevelIconV1>().buffers
            {
                let buffer = buffer.get_handler_mut::<ClientShmWlBuffer>();
                let wl_shm_pool = self
                    .globals
                    .wl_shm
                    .new_send_create_pool(&buffer.pool_fd, buffer.pool_size);
                let wl_buffer = wl_shm_pool.new_send_create_buffer(
                    buffer.offset,
                    buffer.size[0],
                    buffer.size[1],
                    buffer.stride,
                    buffer.format,
                );
                wl_buffer.set_forward_to_client(false);
                vec.push(Icon {
                    logical_size: size / scale.max(1),
                    buffer_size: size,
                    buffer: wl_buffer,
                });
                wl_shm_pool.send_destroy();
            }
            if vec.is_not_empty() {
                icons = Some(vec);
            }
        }
        let client_xdg_toplevel = toplevel.get_handler_mut::<ClientXdgToplevel>();
        let client_xdg_surface = &mut *client_xdg_toplevel
            .xdg_surface
            .get_handler_mut::<ClientXdgSurface>();
        if let Some(prev) = mem::replace(&mut client_xdg_surface.toplevel_icon, icons) {
            for icon in prev {
                icon.buffer.send_destroy();
            }
        }
        for tray in client_xdg_surface.jay_tray_items.values() {
            tray.get_handler_mut::<ProxyJayTrayItemV1>()
                .update_icon(client_xdg_surface, true);
        }
    }
}

impl XdgToplevelIconV1Handler for ClientXdgToplevelIconV1 {
    fn handle_destroy(&mut self, slf: &Rc<XdgToplevelIconV1>) {
        slf.delete_id();
    }

    fn handle_set_name(&mut self, _slf: &Rc<XdgToplevelIconV1>, _icon_name: &str) {
        // nothing
    }

    fn handle_add_buffer(
        &mut self,
        _slf: &Rc<XdgToplevelIconV1>,
        buffer: &Rc<WlBuffer>,
        scale: i32,
    ) {
        let size = buffer.get_handler_mut::<ClientShmWlBuffer>().size;
        self.buffers.insert((size[0], scale), buffer.clone());
    }
}

impl WlShmHandler for ClientWlShm {
    fn handle_create_pool(
        &mut self,
        slf: &Rc<WlShm>,
        id: &Rc<WlShmPool>,
        fd: &Rc<OwnedFd>,
        size: i32,
    ) {
        id.set_handler(ClientWlShmPool {
            fd: fd.clone(),
            size,
        });
        slf.send_create_pool(id, fd, size);
    }
}

impl WlShmPoolHandler for ClientWlShmPool {
    fn handle_create_buffer(
        &mut self,
        slf: &Rc<WlShmPool>,
        id: &Rc<WlBuffer>,
        offset: i32,
        width: i32,
        height: i32,
        stride: i32,
        format: WlShmFormat,
    ) {
        id.set_handler(ClientShmWlBuffer {
            pool_fd: self.fd.clone(),
            pool_size: self.size,
            offset,
            size: [width, height],
            stride,
            format,
        });
        slf.send_create_buffer(id, offset, width, height, stride, format);
    }
}

impl WlBufferHandler for ClientShmWlBuffer {}
