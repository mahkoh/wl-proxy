//! shortcut manager
//!
//! This interface allows a client to get some shell's info.
//!
//! Warning! The protocol described in this file is currently in the testing
//! phase. Backward compatible changes may be added together with the
//! corresponding interface version bump. Backward incompatible changes can
//! only be done by creating a new major version of the extension.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A treeland_shortcut_manager_v1 object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct TreelandShortcutManagerV1 {
    core: ObjectCore,
    handler: HandlerHolder<dyn TreelandShortcutManagerV1Handler>,
}

struct DefaultHandler;

impl TreelandShortcutManagerV1Handler for DefaultHandler { }

impl TreelandShortcutManagerV1 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ObjectInterface = ObjectInterface::TreelandShortcutManagerV1;
    pub const INTERFACE_NAME: &str = "treeland_shortcut_manager_v1";
}

impl TreelandShortcutManagerV1 {
    pub fn set_handler(&self, handler: impl TreelandShortcutManagerV1Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn TreelandShortcutManagerV1Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for TreelandShortcutManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreelandShortcutManagerV1")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl TreelandShortcutManagerV1 {
    /// Since when the register_shortcut_context message is available.
    pub const MSG__REGISTER_SHORTCUT_CONTEXT__SINCE: u32 = 1;

    /// register shortcut key
    ///
    /// The format of the shortcut key is 'Modify+Key', such as 'Ctrl+Alt+T'.
    /// If the format is wrong, the synthesizer will give a "format error". If the shortcut
    /// key is already registered,
    /// the compositor will give a "register error" and issue a destruction to the context.
    ///
    /// # Arguments
    ///
    /// - `key`:
    /// - `id`:
    #[inline]
    pub fn send_register_shortcut_context(
        &self,
        key: &str,
        id: &Rc<TreelandShortcutContextV1>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            key,
            id,
        );
        let arg1_obj = arg1;
        let arg1 = arg1_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        arg1.generate_server_id(arg1_obj.clone())
            .map_err(|e| ObjectError::GenerateServerId("id", e))?;
        let arg1_id = arg1.server_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_shortcut_manager_v1#{}.register_shortcut_context(key: {:?}, id: treeland_shortcut_context_v1#{})\n", id, arg0, arg1_id);
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
        fmt.string(arg0);
        fmt.words([
            arg1_id,
        ]);
        Ok(())
    }
}

/// A message handler for [TreelandShortcutManagerV1] proxies.
pub trait TreelandShortcutManagerV1Handler: Any {
    #[inline]
    fn delete_id(&mut self, slf: &Rc<TreelandShortcutManagerV1>) {
        let _ = slf.core.delete_id();
    }

    /// register shortcut key
    ///
    /// The format of the shortcut key is 'Modify+Key', such as 'Ctrl+Alt+T'.
    /// If the format is wrong, the synthesizer will give a "format error". If the shortcut
    /// key is already registered,
    /// the compositor will give a "register error" and issue a destruction to the context.
    ///
    /// # Arguments
    ///
    /// - `key`:
    /// - `id`:
    #[inline]
    fn handle_register_shortcut_context(
        &mut self,
        _slf: &Rc<TreelandShortcutManagerV1>,
        key: &str,
        id: &Rc<TreelandShortcutContextV1>,
    ) {
        let res = _slf.send_register_shortcut_context(
            key,
            id,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_shortcut_manager_v1.register_shortcut_context message: {}", Report::new(e));
        }
    }
}

impl ObjectPrivate for TreelandShortcutManagerV1 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::TreelandShortcutManagerV1, version),
            handler: Default::default(),
        })
    }

    fn delete_id(self: Rc<Self>) -> Result<(), (ObjectError, Rc<dyn Object>)> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err((ObjectError::HandlerBorrowed, self));
        };
        if let Some(handler) = &mut *handler {
            handler.delete_id(&self);
        } else {
            let _ = self.core.delete_id();
        }
        Ok(())
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err(ObjectError::HandlerBorrowed);
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("key"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("key"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("key"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("key"));
                        };
                        s
                    }
                };
                let Some(&arg1) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("id"));
                };
                offset += 1;
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_shortcut_manager_v1#{}.register_shortcut_context(key: {:?}, id: treeland_shortcut_context_v1#{})\n", client.endpoint.id, msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                let arg1_id = arg1;
                let arg1 = TreelandShortcutContextV1::new(&self.core.state, self.core.version);
                arg1.core().set_client_id(client, arg1_id, arg1.clone())
                    .map_err(|e| ObjectError::SetClientId(arg1_id, "id", e))?;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_register_shortcut_context(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_register_shortcut_context(&self, arg0, arg1);
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
        let Some(mut handler) = self.handler.try_borrow_mut() else {
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
            0 => "register_shortcut_context",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for TreelandShortcutManagerV1 {
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

