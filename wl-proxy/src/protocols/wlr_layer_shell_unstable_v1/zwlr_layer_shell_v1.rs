//! create surfaces that are layers of the desktop
//!
//! Clients can use this interface to assign the surface_layer role to
//! wl_surfaces. Such surfaces are assigned to a "layer" of the output and
//! rendered with a defined z-depth respective to each other. They may also be
//! anchored to the edges and corners of a screen and specify input handling
//! semantics. This interface should be suitable for the implementation of
//! many desktop shell components, and a broad number of other applications
//! that interact with the desktop.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A zwlr_layer_shell_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct ZwlrLayerShellV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn ZwlrLayerShellV1Handler>,
}

struct DefaultHandler;

impl ZwlrLayerShellV1Handler for DefaultHandler { }

impl ConcreteObject for ZwlrLayerShellV1 {
    const XML_VERSION: u32 = 5;
    const INTERFACE: ObjectInterface = ObjectInterface::ZwlrLayerShellV1;
    const INTERFACE_NAME: &str = "zwlr_layer_shell_v1";
}

impl ZwlrLayerShellV1 {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl ZwlrLayerShellV1Handler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn ZwlrLayerShellV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for ZwlrLayerShellV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZwlrLayerShellV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl ZwlrLayerShellV1 {
    /// Since when the get_layer_surface message is available.
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
    pub fn try_send_get_layer_surface(
        &self,
        id: &Rc<ZwlrLayerSurfaceV1>,
        surface: &Rc<WlSurface>,
        output: Option<&Rc<WlOutput>>,
        layer: ZwlrLayerShellV1Layer,
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
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("surface"))),
            Some(id) => id,
        };
        let arg2_id = match arg2 {
            None => 0,
            Some(arg2) => match arg2.server_obj_id.get() {
                None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("output"))),
                Some(id) => id,
            },
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: ZwlrLayerShellV1Layer, arg4: &str) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_layer_shell_v1#{}.get_layer_surface(id: zwlr_layer_surface_v1#{}, surface: wl_surface#{}, output: wl_output#{}, layer: {:?}, namespace: {:?})\n", id, arg0, arg1, arg2, arg3, arg4);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id, arg2_id, arg3, arg4);
        }
        let Some(endpoint) = &self.core.state.server else {
            return Ok(());
        };
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, None);
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
        id: &Rc<ZwlrLayerSurfaceV1>,
        surface: &Rc<WlSurface>,
        output: Option<&Rc<WlOutput>>,
        layer: ZwlrLayerShellV1Layer,
        namespace: &str,
    ) {
        let res = self.try_send_get_layer_surface(
            id,
            surface,
            output,
            layer,
            namespace,
        );
        if let Err(e) = res {
            log_send("zwlr_layer_shell_v1.get_layer_surface", &e);
        }
    }

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
    /// - `surface`:
    /// - `output`:
    /// - `layer`: layer to add this surface to
    /// - `namespace`: namespace for the layer surface
    #[inline]
    pub fn new_try_send_get_layer_surface(
        &self,
        surface: &Rc<WlSurface>,
        output: Option<&Rc<WlOutput>>,
        layer: ZwlrLayerShellV1Layer,
        namespace: &str,
    ) -> Result<Rc<ZwlrLayerSurfaceV1>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_get_layer_surface(
            &id,
            surface,
            output,
            layer,
            namespace,
        )?;
        Ok(id)
    }

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
    /// - `surface`:
    /// - `output`:
    /// - `layer`: layer to add this surface to
    /// - `namespace`: namespace for the layer surface
    #[inline]
    pub fn new_send_get_layer_surface(
        &self,
        surface: &Rc<WlSurface>,
        output: Option<&Rc<WlOutput>>,
        layer: ZwlrLayerShellV1Layer,
        namespace: &str,
    ) -> Rc<ZwlrLayerSurfaceV1> {
        let id = self.core.create_child();
        self.send_get_layer_surface(
            &id,
            surface,
            output,
            layer,
            namespace,
        );
        id
    }

    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 3;

    /// destroy the layer_shell object
    ///
    /// This request indicates that the client will not use the layer_shell
    /// object any more. Objects that have been created through this instance
    /// are not affected.
    #[inline]
    pub fn try_send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= zwlr_layer_shell_v1#{}.destroy()\n", id);
                state.log(args);
            }
            log(&self.core.state, id);
        }
        let Some(endpoint) = &self.core.state.server else {
            return Ok(());
        };
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, None);
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

    /// destroy the layer_shell object
    ///
    /// This request indicates that the client will not use the layer_shell
    /// object any more. Objects that have been created through this instance
    /// are not affected.
    #[inline]
    pub fn send_destroy(
        &self,
    ) {
        let res = self.try_send_destroy(
        );
        if let Err(e) = res {
            log_send("zwlr_layer_shell_v1.destroy", &e);
        }
    }
}

/// A message handler for [`ZwlrLayerShellV1`] proxies.
pub trait ZwlrLayerShellV1Handler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<ZwlrLayerShellV1>) {
        slf.core.delete_id();
    }

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
    fn handle_get_layer_surface(
        &mut self,
        slf: &Rc<ZwlrLayerShellV1>,
        id: &Rc<ZwlrLayerSurfaceV1>,
        surface: &Rc<WlSurface>,
        output: Option<&Rc<WlOutput>>,
        layer: ZwlrLayerShellV1Layer,
        namespace: &str,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_get_layer_surface(
            id,
            surface,
            output,
            layer,
            namespace,
        );
        if let Err(e) = res {
            log_forward("zwlr_layer_shell_v1.get_layer_surface", &e);
        }
    }

    /// destroy the layer_shell object
    ///
    /// This request indicates that the client will not use the layer_shell
    /// object any more. Objects that have been created through this instance
    /// are not affected.
    #[inline]
    fn handle_destroy(
        &mut self,
        slf: &Rc<ZwlrLayerShellV1>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_destroy(
        );
        if let Err(e) = res {
            log_forward("zwlr_layer_shell_v1.destroy", &e);
        }
    }
}

impl ObjectPrivate for ZwlrLayerShellV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::ZwlrLayerShellV1, version),
            handler: Default::default(),
        })
    }

    fn delete_id(self: Rc<Self>) -> Result<(), (ObjectError, Rc<dyn Object>)> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err((ObjectError(ObjectErrorKind::HandlerBorrowed), self));
        };
        if let Some(handler) = &mut *handler {
            handler.delete_id(&self);
        } else {
            self.core.delete_id();
        }
        Ok(())
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err(ObjectError(ObjectErrorKind::HandlerBorrowed));
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("id")));
                };
                offset += 1;
                let Some(&arg1) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("surface")));
                };
                offset += 1;
                let Some(&arg2) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("output")));
                };
                offset += 1;
                let Some(&arg3) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("layer")));
                };
                offset += 1;
                let arg4;
                (arg4, offset) = parse_string::<NonNullString>(msg, offset, "namespace")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                let arg3 = ZwlrLayerShellV1Layer(arg3);
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: u32, arg3: ZwlrLayerShellV1Layer, arg4: &str) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_layer_shell_v1#{}.get_layer_surface(id: zwlr_layer_surface_v1#{}, surface: wl_surface#{}, output: wl_output#{}, layer: {:?}, namespace: {:?})\n", client_id, id, arg0, arg1, arg2, arg3, arg4);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3, arg4);
                }
                let arg0_id = arg0;
                let arg0 = ZwlrLayerSurfaceV1::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("surface", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg2 = if arg2 == 0 {
                    None
                } else {
                    let arg2_id = arg2;
                    let Some(arg2) = client.endpoint.lookup(arg2_id) else {
                        return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg2_id)));
                    };
                    let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<WlOutput>() else {
                        let o = client.endpoint.lookup(arg2_id).unwrap();
                        return Err(ObjectError(ObjectErrorKind::WrongObjectType("output", o.core().interface, ObjectInterface::WlOutput)));
                    };
                    Some(arg2)
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                let arg2 = arg2.as_ref();
                if let Some(handler) = handler {
                    (**handler).handle_get_layer_surface(&self, arg0, arg1, arg2, arg3, arg4);
                } else {
                    DefaultHandler.handle_get_layer_surface(&self, arg0, arg1, arg2, arg3, arg4);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> zwlr_layer_shell_v1#{}.destroy()\n", client_id, id);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0]);
                }
                self.core.handle_client_destroy();
                if let Some(handler) = handler {
                    (**handler).handle_destroy(&self);
                } else {
                    DefaultHandler.handle_destroy(&self);
                }
            }
            n => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
        Ok(())
    }

    fn handle_event(self: Rc<Self>, server: &Endpoint, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err(ObjectError(ObjectErrorKind::HandlerBorrowed));
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            n => {
                let _ = server;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
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

impl Object for ZwlrLayerShellV1 {
    fn core(&self) -> &ObjectCore {
        &self.core
    }

    fn unset_handler(&self) {
        self.handler.set(None);
    }

    fn get_handler_any_ref(&self) -> Result<HandlerRef<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.try_borrow().ok_or(HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(HandlerRef::map(borrowed, |handler| &**handler.as_ref().unwrap() as &dyn Any))
    }

    fn get_handler_any_mut(&self) -> Result<HandlerMut<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.try_borrow_mut().ok_or(HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(HandlerMut::map(borrowed, |handler| &mut **handler.as_mut().unwrap() as &mut dyn Any))
    }
}

impl ZwlrLayerShellV1 {
    /// Since when the error.role enum variant is available.
    pub const ENM__ERROR_ROLE__SINCE: u32 = 1;
    /// Since when the error.invalid_layer enum variant is available.
    pub const ENM__ERROR_INVALID_LAYER__SINCE: u32 = 1;
    /// Since when the error.already_constructed enum variant is available.
    pub const ENM__ERROR_ALREADY_CONSTRUCTED__SINCE: u32 = 1;

    /// Since when the layer.background enum variant is available.
    pub const ENM__LAYER_BACKGROUND__SINCE: u32 = 1;
    /// Since when the layer.bottom enum variant is available.
    pub const ENM__LAYER_BOTTOM__SINCE: u32 = 1;
    /// Since when the layer.top enum variant is available.
    pub const ENM__LAYER_TOP__SINCE: u32 = 1;
    /// Since when the layer.overlay enum variant is available.
    pub const ENM__LAYER_OVERLAY__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZwlrLayerShellV1Error(pub u32);

impl ZwlrLayerShellV1Error {
    /// wl_surface has another role
    pub const ROLE: Self = Self(0);

    /// layer value is invalid
    pub const INVALID_LAYER: Self = Self(1);

    /// wl_surface has a buffer attached or committed
    pub const ALREADY_CONSTRUCTED: Self = Self(2);
}

impl Debug for ZwlrLayerShellV1Error {
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
pub struct ZwlrLayerShellV1Layer(pub u32);

impl ZwlrLayerShellV1Layer {
    pub const BACKGROUND: Self = Self(0);

    pub const BOTTOM: Self = Self(1);

    pub const TOP: Self = Self(2);

    pub const OVERLAY: Self = Self(3);
}

impl Debug for ZwlrLayerShellV1Layer {
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
