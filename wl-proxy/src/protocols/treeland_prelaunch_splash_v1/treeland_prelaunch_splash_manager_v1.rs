//! prelaunch splash screen manager
//!
//! This interface is a manager for creating prelaunch splash screens.
//! A prelaunch splash screen is a placeholder surface that is shown
//! before an application's main window is mapped. This helps to improve
//! the perceived startup time.
//!
//! It is particularly useful for application launchers to provide immediate
//! feedback to the user.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A treeland_prelaunch_splash_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct TreelandPrelaunchSplashManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn TreelandPrelaunchSplashManagerV1Handler>,
}

struct DefaultHandler;

impl TreelandPrelaunchSplashManagerV1Handler for DefaultHandler { }

impl TreelandPrelaunchSplashManagerV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ObjectInterface = ObjectInterface::TreelandPrelaunchSplashManagerV1;
    pub const INTERFACE_NAME: &str = "treeland_prelaunch_splash_manager_v1";
}

impl TreelandPrelaunchSplashManagerV1 {
    pub fn set_handler(&self, handler: impl TreelandPrelaunchSplashManagerV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn TreelandPrelaunchSplashManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for TreelandPrelaunchSplashManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreelandPrelaunchSplashManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl TreelandPrelaunchSplashManagerV1 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the manager
    #[inline]
    pub fn send_destroy(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_prelaunch_splash_manager_v1#{}.destroy()\n", id);
            self.core.state.log(args);
        }
        let endpoint = &self.core.state.server;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, None);
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
        ]);
        self.core.handle_server_destroy();
        Ok(())
    }

    /// Since when the create_splash message is available.
    pub const MSG__CREATE_SPLASH__SINCE: u32 = 1;

    /// create a new splash screen
    ///
    /// Creates a new prelaunch splash screen.
    ///
    /// The `app_id` is a string that identifies the application. The compositor
    /// will use this ID together with `sandbox_engine_name` to match the splash
    /// screen with the actual application window when it appears. This
    /// matching mechanism should also work for XWayland windows.
    ///
    /// Callers MUST provide a non-empty `sandbox_engine_name` string which
    /// identifies the sandboxing/container.
    ///
    /// If there is already an active (not-yet-completed) splash for the same
    /// `sandbox_engine_name` and `app_id`, the compositor will silently ignore
    /// this request (no new splash will be created and no error is raised).
    ///
    /// # Arguments
    ///
    /// - `app_id`: the application ID
    /// - `sandbox_engine_name`: the sandbox engine / security context name (required, non-empty)
    /// - `icon_buffer`: optional icon image as wl_buffer (e.g. wl_shm)
    #[inline]
    pub fn send_create_splash(
        &self,
        app_id: &str,
        sandbox_engine_name: &str,
        icon_buffer: Option<&Rc<WlBuffer>>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            app_id,
            sandbox_engine_name,
            icon_buffer,
        );
        let arg2 = arg2.map(|a| a.core());
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        let arg2_id = match arg2 {
            None => 0,
            Some(arg2) => match arg2.server_obj_id.get() {
                None => return Err(ObjectError::ArgNoServerId("icon_buffer")),
                Some(id) => id,
            },
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_prelaunch_splash_manager_v1#{}.create_splash(app_id: {:?}, sandbox_engine_name: {:?}, icon_buffer: wl_buffer#{})\n", id, arg0, arg1, arg2_id);
            self.core.state.log(args);
        }
        let endpoint = &self.core.state.server;
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
        fmt.string(arg0);
        fmt.string(arg1);
        fmt.words([
            arg2_id,
        ]);
        Ok(())
    }
}

/// A message handler for [TreelandPrelaunchSplashManagerV1] proxies.
pub trait TreelandPrelaunchSplashManagerV1Handler: Any {
    /// destroy the manager
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<TreelandPrelaunchSplashManagerV1>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_prelaunch_splash_manager_v1.destroy message: {}", Report::new(e));
        }
    }

    /// create a new splash screen
    ///
    /// Creates a new prelaunch splash screen.
    ///
    /// The `app_id` is a string that identifies the application. The compositor
    /// will use this ID together with `sandbox_engine_name` to match the splash
    /// screen with the actual application window when it appears. This
    /// matching mechanism should also work for XWayland windows.
    ///
    /// Callers MUST provide a non-empty `sandbox_engine_name` string which
    /// identifies the sandboxing/container.
    ///
    /// If there is already an active (not-yet-completed) splash for the same
    /// `sandbox_engine_name` and `app_id`, the compositor will silently ignore
    /// this request (no new splash will be created and no error is raised).
    ///
    /// # Arguments
    ///
    /// - `app_id`: the application ID
    /// - `sandbox_engine_name`: the sandbox engine / security context name (required, non-empty)
    /// - `icon_buffer`: optional icon image as wl_buffer (e.g. wl_shm)
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn create_splash(
        &mut self,
        _slf: &Rc<TreelandPrelaunchSplashManagerV1>,
        app_id: &str,
        sandbox_engine_name: &str,
        icon_buffer: Option<&Rc<WlBuffer>>,
    ) {
        let res = _slf.send_create_splash(
            app_id,
            sandbox_engine_name,
            icon_buffer,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_prelaunch_splash_manager_v1.create_splash message: {}", Report::new(e));
        }
    }
}

impl ObjectPrivate for TreelandPrelaunchSplashManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::TreelandPrelaunchSplashManagerV1, version),
            handler: Default::default(),
        })
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_prelaunch_splash_manager_v1#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                self.core.handle_client_destroy();
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultHandler.destroy(&self);
                }
            }
            1 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("app_id"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("app_id"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("app_id"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("app_id"));
                        };
                        s
                    }
                };
                let arg1 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("sandbox_engine_name"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("sandbox_engine_name"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("sandbox_engine_name"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("sandbox_engine_name"));
                        };
                        s
                    }
                };
                let Some(&arg2) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("icon_buffer"));
                };
                offset += 1;
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_prelaunch_splash_manager_v1#{}.create_splash(app_id: {:?}, sandbox_engine_name: {:?}, icon_buffer: wl_buffer#{})\n", client.endpoint.id, msg[0], arg0, arg1, arg2);
                    self.core.state.log(args);
                }
                let arg2 = if arg2 == 0 {
                    None
                } else {
                    let arg2_id = arg2;
                    let Some(arg2) = client.endpoint.lookup(arg2_id) else {
                        return Err(ObjectError::NoClientObject(client.endpoint.id, arg2_id));
                    };
                    let Ok(arg2) = (arg2 as Rc<dyn Any>).downcast::<WlBuffer>() else {
                        let o = client.endpoint.lookup(arg2_id).unwrap();
                        return Err(ObjectError::WrongObjectType("icon_buffer", o.core().interface, ObjectInterface::WlBuffer));
                    };
                    Some(arg2)
                };
                let arg2 = arg2.as_ref();
                if let Some(handler) = handler {
                    (**handler).create_splash(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.create_splash(&self, arg0, arg1, arg2);
                }
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
        let Some(mut handler) = self.handler.try_borrow() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
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
            0 => "destroy",
            1 => "create_splash",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for TreelandPrelaunchSplashManagerV1 {
    fn core(&self) -> &ObjectCore {
        &self.core
    }

    fn unset_handler(&self) {
        self.handler.set(None);
    }

    fn get_handler_any_ref(&self) -> Result<Ref<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.handler.try_borrow().map_err(|_| HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(Ref::map(borrowed, |handler| &**handler.as_ref().unwrap() as &dyn Any))
    }

    fn get_handler_any_mut(&self) -> Result<RefMut<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.handler.try_borrow_mut().map_err(|_| HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(RefMut::map(borrowed, |handler| &mut **handler.as_mut().unwrap() as &mut dyn Any))
    }
}

