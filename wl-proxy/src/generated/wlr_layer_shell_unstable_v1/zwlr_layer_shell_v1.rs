//! create surfaces that are layers of the desktop
//!
//! Clients can use this interface to assign the surface_layer role to
//! wl_surfaces. Such surfaces are assigned to a "layer" of the output and
//! rendered with a defined z-depth respective to each other. They may also be
//! anchored to the edges and corners of a screen and specify input handling
//! semantics. This interface should be suitable for the implementation of
//! many desktop shell components, and a broad number of other applications
//! that interact with the desktop.

use crate::generated_helper::prelude::*;
use super::super::all_types::*;

/// A zwlr_layer_shell_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct MetaZwlrLayerShellV1 {
    core: ProxyCore,
    handler: MessageHandlerHolder<dyn MetaZwlrLayerShellV1MessageHandler>,
}

struct DefaultMessageHandler;

impl MetaZwlrLayerShellV1MessageHandler for DefaultMessageHandler { }

impl MetaZwlrLayerShellV1 {
    pub const XML_VERSION: u32 = 5;
}

impl MetaZwlrLayerShellV1 {
    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {
        Rc::new(Self {
            core: ProxyCore::new(state, ProxyInterface::ZwlrLayerShellV1, version),
            handler: Default::default(),
        })
    }
}

impl Debug for MetaZwlrLayerShellV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetaZwlrLayerShellV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl MetaZwlrLayerShellV1 {
    /// Since when the get_layer_surface message is available.
    #[allow(dead_code)]
    pub const MSG__GET_LAYER_SURFACE__SINCE: u32 = 1;

    /// create a layer_surface from a surface
    ///
    /// Create a layer surface for an existing surface. This assigns the role of
    /// layer_surface, or raises a protocol error if another role is already
    /// assigned.
    ///
    /// Creating a layer surface from a wl_surface which has a buffer attached
    /// or committed is a client error, and any attempts by a client to attach
    /// or manipulate a buffer prior to the first layer_surface.configure call
    /// must also be treated as errors.
    ///
    /// After creating a layer_surface object and setting it up, the client
    /// must perform an initial commit without any buffer attached.
    /// The compositor will reply with a layer_surface.configure event.
    /// The client must acknowledge it and is then allowed to attach a buffer
    /// to map the surface.
    ///
    /// You may pass NULL for output to allow the compositor to decide which
    /// output to use. Generally this will be the one that the user most
    /// recently interacted with.
    ///
    /// Clients can specify a namespace that defines the purpose of the layer
    /// surface.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    /// - `output`:
    /// - `layer`: layer to add this surface to
    /// - `namespace`: namespace for the layer surface
    #[inline]
    pub fn send_get_layer_surface(
        &self,
        id: &Rc<MetaZwlrLayerSurfaceV1>,
        surface: &Rc<MetaWlSurface>,
        output: Option<&Rc<MetaWlOutput>>,
        layer: MetaZwlrLayerShellV1Layer,
        namespace: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
        ) = (
            id,
            surface,
            output,
            layer,
            namespace,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let arg1 = arg1.core();
        let arg2 = arg2.map(|a| a.core());
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError::ArgNoServerId("surface")),
            Some(id) => id,
        };
        let arg2_id = match arg2 {
            None => 0,
            Some(arg2) => match arg2.server_obj_id.get() {
                None => return Err(ObjectError::ArgNoServerId("output")),
                Some(id) => id,
            },
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        eprintln!("server      <= zwlr_layer_shell_v1#{}.get_layer_surface(id: zwlr_layer_surface_v1#{}, surface: wl_surface#{}, output: wl_output#{}, layer: {:?}, namespace: {:?})", id, arg0_id, arg1_id, arg2_id, arg3, arg4);
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
            arg0_id,
            arg1_id,
            arg2_id,
            arg3.0,
        ]);
        fmt.string(arg4);
        Ok(())
    }

    /// Since when the destroy message is available.
    #[allow(dead_code)]
    pub const MSG__DESTROY__SINCE: u32 = 3;

    /// destroy the layer_shell object
    ///
    /// This request indicates that the client will not use the layer_shell
    /// object any more. Objects that have been created through this instance
    /// are not affected.
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        eprintln!("server      <= zwlr_layer_shell_v1#{}.destroy()", id);
        let endpoint = &self.core.state.server;
        if !endpoint.has_outgoing.replace(true) {
            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }
}

/// A message handler for [ZwlrLayerShellV1] proxies.
#[allow(dead_code)]
pub trait MetaZwlrLayerShellV1MessageHandler {
    /// create a layer_surface from a surface
    ///
    /// Create a layer surface for an existing surface. This assigns the role of
    /// layer_surface, or raises a protocol error if another role is already
    /// assigned.
    ///
    /// Creating a layer surface from a wl_surface which has a buffer attached
    /// or committed is a client error, and any attempts by a client to attach
    /// or manipulate a buffer prior to the first layer_surface.configure call
    /// must also be treated as errors.
    ///
    /// After creating a layer_surface object and setting it up, the client
    /// must perform an initial commit without any buffer attached.
    /// The compositor will reply with a layer_surface.configure event.
    /// The client must acknowledge it and is then allowed to attach a buffer
    /// to map the surface.
    ///
    /// You may pass NULL for output to allow the compositor to decide which
    /// output to use. Generally this will be the one that the user most
    /// recently interacted with.
    ///
    /// Clients can specify a namespace that defines the purpose of the layer
    /// surface.
    ///
    /// # Arguments
    ///
    /// - `id`:
    /// - `surface`:
    /// - `output`:
    /// - `layer`: layer to add this surface to
    /// - `namespace`: namespace for the layer surface
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn get_layer_surface(
        &mut self,
        _slf: &Rc<MetaZwlrLayerShellV1>,
        id: &Rc<MetaZwlrLayerSurfaceV1>,
        surface: &Rc<MetaWlSurface>,
        output: Option<&Rc<MetaWlOutput>>,
        layer: MetaZwlrLayerShellV1Layer,
        namespace: &str,
    ) {
        let res = _slf.send_get_layer_surface(
            id,
            surface,
            output,
            layer,
            namespace,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_layer_shell_v1.get_layer_surface message: {}", Report::new(e));
        }
    }

    /// destroy the layer_shell object
    ///
    /// This request indicates that the client will not use the layer_shell
    /// object any more. Objects that have been created through this instance
    /// are not affected.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<MetaZwlrLayerShellV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a zwlr_layer_shell_v1.destroy message: {}", Report::new(e));
        }
    }
}

impl Proxy for MetaZwlrLayerShellV1 {
    fn core(&self) -> &ProxyCore {
        &self.core
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            0 => {
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("id"));
                };
                offset += 1;
                let Some(&arg1) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("surface"));
                };
                offset += 1;
                let Some(&arg2) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("output"));
                };
                offset += 1;
                let Some(&arg3) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("layer"));
                };
                offset += 1;
                let arg4 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("namespace"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("namespace"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("namespace"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("namespace"));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                let arg3 = MetaZwlrLayerShellV1Layer(arg3);
                eprintln!("client#{:04} -> zwlr_layer_shell_v1#{}.get_layer_surface(id: zwlr_layer_surface_v1#{}, surface: wl_surface#{}, output: wl_output#{}, layer: {:?}, namespace: {:?})", client.endpoint.id, msg[0], arg0, arg1, arg2, arg3, arg4);
                let arg0_id = arg0;
                let arg0 = MetaZwlrLayerSurfaceV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError::SetClientId(arg0_id, "id", e))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError::NoClientObject(client.endpoint.id, arg1_id));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<MetaWlSurface>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError::WrongObjectType("surface", o.core().interface, ProxyInterface::WlSurface));
                };
                let arg2 = if arg2 == 0 {
                    None
                } else {
                    let arg2_id = arg2;
                    let Some(arg2) = client.endpoint.lookup(arg2_id) else {
                        return Err(ObjectError::NoClientObject(client.endpoint.id, arg2_id));
                    };
                    let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<MetaWlOutput>() else {
                        let o = client.endpoint.lookup(arg2_id).unwrap();
                        return Err(ObjectError::WrongObjectType("output", o.core().interface, ProxyInterface::WlOutput));
                    };
                    Some(arg2)
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                let arg2 = arg2.as_ref();
                if let Some(handler) = handler {
                    (**handler).get_layer_surface(&self, arg0, arg1, arg2, arg3, arg4);
                } else {
                    DefaultMessageHandler.get_layer_surface(&self, arg0, arg1, arg2, arg3, arg4);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                eprintln!("client#{:04} -> zwlr_layer_shell_v1#{}.destroy()", client.endpoint.id, msg[0]);
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultMessageHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
            }
            n => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError::UnknownMessageId(n));
            }
        }
        Ok(())
    }

    fn handle_event(self: Rc<Self>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let handler = &mut *self.handler.borrow();
        match msg[1] & 0xffff {
            n => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError::UnknownMessageId(n));
            }
        }
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "get_layer_surface",
            1 => "destroy",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl MetaZwlrLayerShellV1 {
    /// Since when the error.role enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_ROLE__SINCE: u32 = 1;
    /// Since when the error.invalid_layer enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_LAYER__SINCE: u32 = 1;
    /// Since when the error.already_constructed enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_ALREADY_CONSTRUCTED__SINCE: u32 = 1;

    /// Since when the layer.background enum variant is available.
    #[allow(dead_code)]
    pub const ENM__LAYER_BACKGROUND__SINCE: u32 = 1;
    /// Since when the layer.bottom enum variant is available.
    #[allow(dead_code)]
    pub const ENM__LAYER_BOTTOM__SINCE: u32 = 1;
    /// Since when the layer.top enum variant is available.
    #[allow(dead_code)]
    pub const ENM__LAYER_TOP__SINCE: u32 = 1;
    /// Since when the layer.overlay enum variant is available.
    #[allow(dead_code)]
    pub const ENM__LAYER_OVERLAY__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaZwlrLayerShellV1Error(pub u32);

impl MetaZwlrLayerShellV1Error {
    /// wl_surface has another role
    #[allow(dead_code)]
    pub const ROLE: Self = Self(0);

    /// layer value is invalid
    #[allow(dead_code)]
    pub const INVALID_LAYER: Self = Self(1);

    /// wl_surface has a buffer attached or committed
    #[allow(dead_code)]
    pub const ALREADY_CONSTRUCTED: Self = Self(2);
}

impl Debug for MetaZwlrLayerShellV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ROLE => "ROLE",
            Self::INVALID_LAYER => "INVALID_LAYER",
            Self::ALREADY_CONSTRUCTED => "ALREADY_CONSTRUCTED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// available layers for surfaces
///
/// These values indicate which layers a surface can be rendered in. They
/// are ordered by z depth, bottom-most first. Traditional shell surfaces
/// will typically be rendered between the bottom and top layers.
/// Fullscreen shell surfaces are typically rendered at the top layer.
/// Multiple surfaces can share a single layer, and ordering within a
/// single layer is undefined.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct MetaZwlrLayerShellV1Layer(pub u32);

impl MetaZwlrLayerShellV1Layer {
    #[allow(dead_code)]
    pub const BACKGROUND: Self = Self(0);

    #[allow(dead_code)]
    pub const BOTTOM: Self = Self(1);

    #[allow(dead_code)]
    pub const TOP: Self = Self(2);

    #[allow(dead_code)]
    pub const OVERLAY: Self = Self(3);
}

impl Debug for MetaZwlrLayerShellV1Layer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::BACKGROUND => "BACKGROUND",
            Self::BOTTOM => "BOTTOM",
            Self::TOP => "TOP",
            Self::OVERLAY => "OVERLAY",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
