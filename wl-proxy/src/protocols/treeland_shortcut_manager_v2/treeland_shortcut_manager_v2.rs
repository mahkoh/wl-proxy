//! global shortcuts manager for treeland
//!
//! This interface allows privileged clients to register global shortcuts.
//!
//! In treeland, global shortcuts are managed in a per-user context.
//! Shortcuts for different users are isolated, and will not interfere with each other.
//! This allows multiple users to use their own set of global Shortcuts
//! on the same system without conflicts.
//! This behavior is transparent to the clients of this interface (i.e 
//! the user context used by this protocol is the same as that of the client.)
//!
//! Warning! The protocol described in this file is currently in the testing
//! phase. Backward compatible changes may be added together with the
//! corresponding interface version bump. Backward incompatible changes can
//! only be done by creating a new major version of the extension.

use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A treeland_shortcut_manager_v2 proxy.
///
/// See the documentation of [the module][self] for the interface description.
pub struct TreelandShortcutManagerV2 {
    core: ProxyCore,
    handler: HandlerHolder<dyn TreelandShortcutManagerV2Handler>,
}

struct DefaultHandler;

impl TreelandShortcutManagerV2Handler for DefaultHandler { }

impl TreelandShortcutManagerV2 {
    pub const XML_VERSION: u32 = 1;
    pub const INTERFACE: ProxyInterface = ProxyInterface::TreelandShortcutManagerV2;
    pub const INTERFACE_NAME: &str = "treeland_shortcut_manager_v2";
}

impl TreelandShortcutManagerV2 {
    pub fn set_handler(&self, handler: impl TreelandShortcutManagerV2Handler + 'static) {
        self.set_boxed_handler(Box::new(handler));
    }

    pub fn set_boxed_handler(&self, handler: Box<dyn TreelandShortcutManagerV2Handler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for TreelandShortcutManagerV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreelandShortcutManagerV2")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl TreelandShortcutManagerV2 {
    /// Since when the destroy message is available.
    pub const MSG__DESTROY__SINCE: u32 = 1;

    /// destroy the shortcut manager
    ///
    /// Destroy the shortcut manager.
    /// Existing shortcuts created through this interface remain valid.
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
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_shortcut_manager_v2#{}.destroy()\n", id);
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

    /// Since when the acquire message is available.
    pub const MSG__ACQUIRE__SINCE: u32 = 1;

    /// acquire the shortcut manager
    ///
    /// Acquire the shortcut manager for the current client.
    ///
    /// This request must be sent before any bind/unbind request can be performed.
    ///
    /// Only one client hold exclusive control of the shortcut manager at a time,
    /// for a given session.
    /// If the shortcut manager is already acquired by another client, an protocol error
    #[inline]
    pub fn send_acquire(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_shortcut_manager_v2#{}.acquire()\n", id);
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
        Ok(())
    }

    /// Since when the bind_key message is available.
    pub const MSG__BIND_KEY__SINCE: u32 = 1;

    /// bind a key sequence to a compositor action
    ///
    /// Bind a key sequence to a compositor action.
    ///
    /// The key sequence is specified in the string format used by QKeySequence,
    /// see https://doc.qt.io/qt-6/qkeysequence.html#toString for details.
    ///
    /// Sending this request without first acquiring the shortcut manager
    /// will result in a `not_acquired` protocol error.
    ///
    /// The name argument must be unique among all existing bindings.
    /// If a binding with the same name already exists, the bind_key request will fail.
    ///
    /// The action argument specifies the compositor action to be executed
    /// when the key sequence is activated.
    ///
    /// The protocol provides three keybinding modes:
    /// - key_release: the action is triggered when the key sequence is released.
    /// - key_press: the action is triggered when the key sequence is pressed.
    /// - key_press_repeat: the action is triggered when the key sequence is pressed,
    ///   and repeatedly triggered if the key sequence is held down.
    ///
    /// If a binding with the same key sequence and action already exists,
    /// the bind_key request will fail.
    ///
    /// Note that the binding will not take effect until a commit request is sent.
    ///
    /// # Arguments
    ///
    /// - `name`: unique name for the keybinding
    /// - `key`: key sequence for the keybinding
    /// - `mode`: mode for the keybinding
    /// - `action`: compositor action to be executed
    #[inline]
    pub fn send_bind_key(
        &self,
        name: &str,
        key: &str,
        mode: TreelandShortcutManagerV2KeybindMode,
        action: TreelandShortcutManagerV2Action,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            name,
            key,
            mode,
            action,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_shortcut_manager_v2#{}.bind_key(name: {:?}, key: {:?}, mode: {:?}, action: {:?})\n", id, arg0, arg1, arg2, arg3);
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
            2,
        ]);
        fmt.string(arg0);
        fmt.string(arg1);
        fmt.words([
            arg2.0,
            arg3.0,
        ]);
        Ok(())
    }

    /// Since when the bind_swipe_gesture message is available.
    pub const MSG__BIND_SWIPE_GESTURE__SINCE: u32 = 1;

    /// bind a swipe gesture to a compositor action
    ///
    /// Bind a swipe gesture to a compositor action.
    ///
    /// Sending this request without first acquiring the shortcut manager
    /// will result in a `not_acquired` protocol error.
    ///
    /// The name argument must be unique among all existing bindings.
    /// If a binding with the same name already exists, the bind_swipe_gesture request will fail.
    ///
    /// The direction argument specifies the direction towards which the swipe gesture must be performed.
    /// If this argument is not one of the defined enum values, the bind_swipe_gesture request will fail.
    ///
    /// The action argument specifies the compositor action to be executed
    /// when the swipe gesture is activated.
    /// If a binding with the same gesture and action already exists,
    /// the bind_swipe_gesture request will fail.
    ///
    /// Note that the binding will not take effect until a commit request is sent.
    ///
    /// # Arguments
    ///
    /// - `name`: unique name for the swipe gesture
    /// - `finger`: number of fingers required for the swipe gesture
    /// - `direction`: direction of the swipe gesture
    /// - `action`: compositor action to be executed
    #[inline]
    pub fn send_bind_swipe_gesture(
        &self,
        name: &str,
        finger: u32,
        direction: TreelandShortcutManagerV2Direction,
        action: TreelandShortcutManagerV2Action,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
        ) = (
            name,
            finger,
            direction,
            action,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_shortcut_manager_v2#{}.bind_swipe_gesture(name: {:?}, finger: {}, direction: {:?}, action: {:?})\n", id, arg0, arg1, arg2, arg3);
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
            3,
        ]);
        fmt.string(arg0);
        fmt.words([
            arg1,
            arg2.0,
            arg3.0,
        ]);
        Ok(())
    }

    /// Since when the bind_hold_gesture message is available.
    pub const MSG__BIND_HOLD_GESTURE__SINCE: u32 = 1;

    /// bind a hold gesture to a compositor action
    ///
    /// Bind a hold gesture to a compositor action.
    ///
    /// Sending this request without first acquiring the shortcut manager
    /// will result in a `not_acquired` protocol error.
    ///
    /// The name argument must be unique among all existing bindings.
    /// If a binding with the same name already exists, the bind_hold_gesture request will fail.
    ///
    /// The action argument specifies the compositor action to be executed
    /// when the hold gesture is activated.
    /// If a binding with the same gesture and action already exists,
    /// the bind_hold_gesture request will fail.
    ///
    /// Note that the binding will not take effect until a commit request is sent.
    ///
    /// # Arguments
    ///
    /// - `name`: unique name for the hold gesture
    /// - `finger`: number of fingers required for the hold gesture
    /// - `action`: compositor action to be executed
    #[inline]
    pub fn send_bind_hold_gesture(
        &self,
        name: &str,
        finger: u32,
        action: TreelandShortcutManagerV2Action,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            name,
            finger,
            action,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_shortcut_manager_v2#{}.bind_hold_gesture(name: {:?}, finger: {}, action: {:?})\n", id, arg0, arg1, arg2);
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
            4,
        ]);
        fmt.string(arg0);
        fmt.words([
            arg1,
            arg2.0,
        ]);
        Ok(())
    }

    /// Since when the commit message is available.
    pub const MSG__COMMIT__SINCE: u32 = 1;

    /// commit the pending bindings
    ///
    /// Commit the pending bindings.
    ///
    /// This request applies all the bind_key, bind_swipe_gesture and bind_hold_gesture
    /// requests sent since the last commit.
    ///
    /// After processing this request, the compositor will emit a `commit_status` event
    /// if the commit was successful or `commit_failure` event if the commit failed.
    ///
    /// On a successful commit, all the pending bindings will take effect.
    /// On a failed commit, none of the pending bindings will take effect.
    ///
    /// Sending this request without first acquiring the shortcut manager
    /// will result in a `not_acquired` protocol error.
    ///
    /// Sending further commit requests before `commit_success` or `commit_failure`
    /// is sent is a protocol error.
    #[inline]
    pub fn send_commit(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_shortcut_manager_v2#{}.commit()\n", id);
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
            5,
        ]);
        Ok(())
    }

    /// Since when the unbind message is available.
    pub const MSG__UNBIND__SINCE: u32 = 1;

    /// remove an existing binding
    ///
    /// Remove an existing binding.
    ///
    /// The binding to be removed is identified by its unique name.
    /// If no binding with the specified name exists, the unbind request has no effect.
    ///
    /// # Arguments
    ///
    /// - `name`: unique name of the binding to be removed
    #[inline]
    pub fn send_unbind(
        &self,
        name: &str,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            name,
        );
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError::ReceiverNoServerId);
        };
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= treeland_shortcut_manager_v2#{}.unbind(name: {:?})\n", id, arg0);
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
            6,
        ]);
        fmt.string(arg0);
        Ok(())
    }

    /// Since when the activated message is available.
    pub const MSG__ACTIVATED__SINCE: u32 = 1;

    /// a shortcut has been activated
    ///
    /// This event is emitted when a binding registered with action `notify` is activated.
    ///
    /// If the binding is activated due to auto-repeat, the repeat argument will be non-zero.
    ///
    /// # Arguments
    ///
    /// - `name`: binding id of the activated shortcut
    /// - `repeat`: indicates whether the shortcut activation is due to auto-repeat
    #[inline]
    pub fn send_activated(
        &self,
        name: &str,
        repeat: u32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            name,
            repeat,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_shortcut_manager_v2#{}.activated(name: {:?}, repeat: {})\n", client.endpoint.id, id, arg0, arg1);
            self.core.state.log(args);
        }
        let endpoint = &client.endpoint;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, Some(client));
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
            arg1,
        ]);
        Ok(())
    }

    /// Since when the commit_success message is available.
    pub const MSG__COMMIT_SUCCESS__SINCE: u32 = 1;

    /// the last commit was successful
    ///
    /// This event is emitted in response to a commit request,
    /// indicating that the commit was successful.
    #[inline]
    pub fn send_commit_success(
        &self,
    ) -> Result<(), ObjectError> {
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_shortcut_manager_v2#{}.commit_success()\n", client.endpoint.id, id);
            self.core.state.log(args);
        }
        let endpoint = &client.endpoint;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, Some(client));
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
        ]);
        Ok(())
    }

    /// Since when the commit_failure message is available.
    pub const MSG__COMMIT_FAILURE__SINCE: u32 = 1;

    /// the last commit has failed
    ///
    /// This event is emitted in response to a commit request,
    /// indicating that the commit has failed.
    ///
    /// The error argument indicates the first error that caused the commit to fail.
    ///
    /// # Arguments
    ///
    /// - `name`: binding name that caused the failure
    /// - `error`: error code indicating the reason of the failure
    #[inline]
    pub fn send_commit_failure(
        &self,
        name: &str,
        error: TreelandShortcutManagerV2BindError,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            name,
            error,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError::ReceiverNoClient);
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if self.core.state.log {
            let (millis, micros) = time_since_epoch();
            let prefix = &self.core.state.log_prefix;
            let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= treeland_shortcut_manager_v2#{}.commit_failure(name: {:?}, error: {:?})\n", client.endpoint.id, id, arg0, arg1);
            self.core.state.log(args);
        }
        let endpoint = &client.endpoint;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, Some(client));
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            2,
        ]);
        fmt.string(arg0);
        fmt.words([
            arg1.0,
        ]);
        Ok(())
    }
}

/// A message handler for [TreelandShortcutManagerV2] proxies.
pub trait TreelandShortcutManagerV2Handler: Any {
    /// destroy the shortcut manager
    ///
    /// Destroy the shortcut manager.
    /// Existing shortcuts created through this interface remain valid.
    #[inline]
    fn destroy(
        &mut self,
        _slf: &Rc<TreelandShortcutManagerV2>,
    ) {
        let res = _slf.send_destroy(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_shortcut_manager_v2.destroy message: {}", Report::new(e));
        }
    }

    /// acquire the shortcut manager
    ///
    /// Acquire the shortcut manager for the current client.
    ///
    /// This request must be sent before any bind/unbind request can be performed.
    ///
    /// Only one client hold exclusive control of the shortcut manager at a time,
    /// for a given session.
    /// If the shortcut manager is already acquired by another client, an protocol error
    #[inline]
    fn acquire(
        &mut self,
        _slf: &Rc<TreelandShortcutManagerV2>,
    ) {
        let res = _slf.send_acquire(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_shortcut_manager_v2.acquire message: {}", Report::new(e));
        }
    }

    /// bind a key sequence to a compositor action
    ///
    /// Bind a key sequence to a compositor action.
    ///
    /// The key sequence is specified in the string format used by QKeySequence,
    /// see https://doc.qt.io/qt-6/qkeysequence.html#toString for details.
    ///
    /// Sending this request without first acquiring the shortcut manager
    /// will result in a `not_acquired` protocol error.
    ///
    /// The name argument must be unique among all existing bindings.
    /// If a binding with the same name already exists, the bind_key request will fail.
    ///
    /// The action argument specifies the compositor action to be executed
    /// when the key sequence is activated.
    ///
    /// The protocol provides three keybinding modes:
    /// - key_release: the action is triggered when the key sequence is released.
    /// - key_press: the action is triggered when the key sequence is pressed.
    /// - key_press_repeat: the action is triggered when the key sequence is pressed,
    ///   and repeatedly triggered if the key sequence is held down.
    ///
    /// If a binding with the same key sequence and action already exists,
    /// the bind_key request will fail.
    ///
    /// Note that the binding will not take effect until a commit request is sent.
    ///
    /// # Arguments
    ///
    /// - `name`: unique name for the keybinding
    /// - `key`: key sequence for the keybinding
    /// - `mode`: mode for the keybinding
    /// - `action`: compositor action to be executed
    #[inline]
    fn bind_key(
        &mut self,
        _slf: &Rc<TreelandShortcutManagerV2>,
        name: &str,
        key: &str,
        mode: TreelandShortcutManagerV2KeybindMode,
        action: TreelandShortcutManagerV2Action,
    ) {
        let res = _slf.send_bind_key(
            name,
            key,
            mode,
            action,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_shortcut_manager_v2.bind_key message: {}", Report::new(e));
        }
    }

    /// bind a swipe gesture to a compositor action
    ///
    /// Bind a swipe gesture to a compositor action.
    ///
    /// Sending this request without first acquiring the shortcut manager
    /// will result in a `not_acquired` protocol error.
    ///
    /// The name argument must be unique among all existing bindings.
    /// If a binding with the same name already exists, the bind_swipe_gesture request will fail.
    ///
    /// The direction argument specifies the direction towards which the swipe gesture must be performed.
    /// If this argument is not one of the defined enum values, the bind_swipe_gesture request will fail.
    ///
    /// The action argument specifies the compositor action to be executed
    /// when the swipe gesture is activated.
    /// If a binding with the same gesture and action already exists,
    /// the bind_swipe_gesture request will fail.
    ///
    /// Note that the binding will not take effect until a commit request is sent.
    ///
    /// # Arguments
    ///
    /// - `name`: unique name for the swipe gesture
    /// - `finger`: number of fingers required for the swipe gesture
    /// - `direction`: direction of the swipe gesture
    /// - `action`: compositor action to be executed
    #[inline]
    fn bind_swipe_gesture(
        &mut self,
        _slf: &Rc<TreelandShortcutManagerV2>,
        name: &str,
        finger: u32,
        direction: TreelandShortcutManagerV2Direction,
        action: TreelandShortcutManagerV2Action,
    ) {
        let res = _slf.send_bind_swipe_gesture(
            name,
            finger,
            direction,
            action,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_shortcut_manager_v2.bind_swipe_gesture message: {}", Report::new(e));
        }
    }

    /// bind a hold gesture to a compositor action
    ///
    /// Bind a hold gesture to a compositor action.
    ///
    /// Sending this request without first acquiring the shortcut manager
    /// will result in a `not_acquired` protocol error.
    ///
    /// The name argument must be unique among all existing bindings.
    /// If a binding with the same name already exists, the bind_hold_gesture request will fail.
    ///
    /// The action argument specifies the compositor action to be executed
    /// when the hold gesture is activated.
    /// If a binding with the same gesture and action already exists,
    /// the bind_hold_gesture request will fail.
    ///
    /// Note that the binding will not take effect until a commit request is sent.
    ///
    /// # Arguments
    ///
    /// - `name`: unique name for the hold gesture
    /// - `finger`: number of fingers required for the hold gesture
    /// - `action`: compositor action to be executed
    #[inline]
    fn bind_hold_gesture(
        &mut self,
        _slf: &Rc<TreelandShortcutManagerV2>,
        name: &str,
        finger: u32,
        action: TreelandShortcutManagerV2Action,
    ) {
        let res = _slf.send_bind_hold_gesture(
            name,
            finger,
            action,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_shortcut_manager_v2.bind_hold_gesture message: {}", Report::new(e));
        }
    }

    /// commit the pending bindings
    ///
    /// Commit the pending bindings.
    ///
    /// This request applies all the bind_key, bind_swipe_gesture and bind_hold_gesture
    /// requests sent since the last commit.
    ///
    /// After processing this request, the compositor will emit a `commit_status` event
    /// if the commit was successful or `commit_failure` event if the commit failed.
    ///
    /// On a successful commit, all the pending bindings will take effect.
    /// On a failed commit, none of the pending bindings will take effect.
    ///
    /// Sending this request without first acquiring the shortcut manager
    /// will result in a `not_acquired` protocol error.
    ///
    /// Sending further commit requests before `commit_success` or `commit_failure`
    /// is sent is a protocol error.
    #[inline]
    fn commit(
        &mut self,
        _slf: &Rc<TreelandShortcutManagerV2>,
    ) {
        let res = _slf.send_commit(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_shortcut_manager_v2.commit message: {}", Report::new(e));
        }
    }

    /// remove an existing binding
    ///
    /// Remove an existing binding.
    ///
    /// The binding to be removed is identified by its unique name.
    /// If no binding with the specified name exists, the unbind request has no effect.
    ///
    /// # Arguments
    ///
    /// - `name`: unique name of the binding to be removed
    #[inline]
    fn unbind(
        &mut self,
        _slf: &Rc<TreelandShortcutManagerV2>,
        name: &str,
    ) {
        let res = _slf.send_unbind(
            name,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_shortcut_manager_v2.unbind message: {}", Report::new(e));
        }
    }

    /// a shortcut has been activated
    ///
    /// This event is emitted when a binding registered with action `notify` is activated.
    ///
    /// If the binding is activated due to auto-repeat, the repeat argument will be non-zero.
    ///
    /// # Arguments
    ///
    /// - `name`: binding id of the activated shortcut
    /// - `repeat`: indicates whether the shortcut activation is due to auto-repeat
    #[inline]
    fn activated(
        &mut self,
        _slf: &Rc<TreelandShortcutManagerV2>,
        name: &str,
        repeat: u32,
    ) {
        let res = _slf.send_activated(
            name,
            repeat,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_shortcut_manager_v2.activated message: {}", Report::new(e));
        }
    }

    /// the last commit was successful
    ///
    /// This event is emitted in response to a commit request,
    /// indicating that the commit was successful.
    #[inline]
    fn commit_success(
        &mut self,
        _slf: &Rc<TreelandShortcutManagerV2>,
    ) {
        let res = _slf.send_commit_success(
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_shortcut_manager_v2.commit_success message: {}", Report::new(e));
        }
    }

    /// the last commit has failed
    ///
    /// This event is emitted in response to a commit request,
    /// indicating that the commit has failed.
    ///
    /// The error argument indicates the first error that caused the commit to fail.
    ///
    /// # Arguments
    ///
    /// - `name`: binding name that caused the failure
    /// - `error`: error code indicating the reason of the failure
    #[inline]
    fn commit_failure(
        &mut self,
        _slf: &Rc<TreelandShortcutManagerV2>,
        name: &str,
        error: TreelandShortcutManagerV2BindError,
    ) {
        let res = _slf.send_commit_failure(
            name,
            error,
        );
        if let Err(e) = res {
            log::warn!("Could not forward a treeland_shortcut_manager_v2.commit_failure message: {}", Report::new(e));
        }
    }
}

impl ProxyPrivate for TreelandShortcutManagerV2 {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ProxyCore::new(state, slf.clone(), ProxyInterface::TreelandShortcutManagerV2, version),
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
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_shortcut_manager_v2#{}.destroy()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).destroy(&self);
                } else {
                    DefaultHandler.destroy(&self);
                }
                self.core.handle_client_destroy();
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_shortcut_manager_v2#{}.acquire()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).acquire(&self);
                } else {
                    DefaultHandler.acquire(&self);
                }
            }
            2 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("name"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("name"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("name"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("name"));
                        };
                        s
                    }
                };
                let arg1 = {
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
                let Some(&arg2) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("mode"));
                };
                offset += 1;
                let Some(&arg3) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("action"));
                };
                offset += 1;
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                let arg2 = TreelandShortcutManagerV2KeybindMode(arg2);
                let arg3 = TreelandShortcutManagerV2Action(arg3);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_shortcut_manager_v2#{}.bind_key(name: {:?}, key: {:?}, mode: {:?}, action: {:?})\n", client.endpoint.id, msg[0], arg0, arg1, arg2, arg3);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).bind_key(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.bind_key(&self, arg0, arg1, arg2, arg3);
                }
            }
            3 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("name"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("name"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("name"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("name"));
                        };
                        s
                    }
                };
                let Some(&arg1) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("finger"));
                };
                offset += 1;
                let Some(&arg2) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("direction"));
                };
                offset += 1;
                let Some(&arg3) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("action"));
                };
                offset += 1;
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                let arg2 = TreelandShortcutManagerV2Direction(arg2);
                let arg3 = TreelandShortcutManagerV2Action(arg3);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_shortcut_manager_v2#{}.bind_swipe_gesture(name: {:?}, finger: {}, direction: {:?}, action: {:?})\n", client.endpoint.id, msg[0], arg0, arg1, arg2, arg3);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).bind_swipe_gesture(&self, arg0, arg1, arg2, arg3);
                } else {
                    DefaultHandler.bind_swipe_gesture(&self, arg0, arg1, arg2, arg3);
                }
            }
            4 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("name"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("name"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("name"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("name"));
                        };
                        s
                    }
                };
                let Some(&arg1) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("finger"));
                };
                offset += 1;
                let Some(&arg2) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("action"));
                };
                offset += 1;
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                let arg2 = TreelandShortcutManagerV2Action(arg2);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_shortcut_manager_v2#{}.bind_hold_gesture(name: {:?}, finger: {}, action: {:?})\n", client.endpoint.id, msg[0], arg0, arg1, arg2);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).bind_hold_gesture(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.bind_hold_gesture(&self, arg0, arg1, arg2);
                }
            }
            5 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_shortcut_manager_v2#{}.commit()\n", client.endpoint.id, msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).commit(&self);
                } else {
                    DefaultHandler.commit(&self);
                }
            }
            6 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("name"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("name"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("name"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("name"));
                        };
                        s
                    }
                };
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> treeland_shortcut_manager_v2#{}.unbind(name: {:?})\n", client.endpoint.id, msg[0], arg0);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).unbind(&self, arg0);
                } else {
                    DefaultHandler.unbind(&self, arg0);
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
            0 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("name"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("name"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("name"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("name"));
                        };
                        s
                    }
                };
                let Some(&arg1) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("repeat"));
                };
                offset += 1;
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_shortcut_manager_v2#{}.activated(name: {:?}, repeat: {})\n", msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).activated(&self, arg0, arg1);
                } else {
                    DefaultHandler.activated(&self, arg0, arg1);
                }
            }
            1 => {
                if msg.len() != 2 {
                    return Err(ObjectError::WrongMessageSize(msg.len() as u32 * 4, 8));
                }
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_shortcut_manager_v2#{}.commit_success()\n", msg[0]);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).commit_success(&self);
                } else {
                    DefaultHandler.commit_success(&self);
                }
            }
            2 => {
                let mut offset = 2;
                let arg0 = {
                    let Some(&len) = msg.get(offset) else {
                        return Err(ObjectError::MissingArgument("name"));
                    };
                    offset += 1;
                    let len = len as usize;
                    let words = ((len as u64 + 3) / 4) as usize;
                    if offset + words > msg.len() {
                        return Err(ObjectError::MissingArgument("name"));
                    }
                    let start = offset;
                    offset += words;
                    let bytes = &uapi::as_bytes(&msg[start..])[..len];
                    if bytes.is_empty() {
                        return Err(ObjectError::NullString("name"));
                    } else {
                        let Ok(s) = str::from_utf8(&bytes[..len-1]) else {
                            return Err(ObjectError::NonUtf8("name"));
                        };
                        s
                    }
                };
                let Some(&arg1) = msg.get(offset) else {
                    return Err(ObjectError::MissingArgument("error"));
                };
                offset += 1;
                if offset != msg.len() {
                    return Err(ObjectError::TrailingBytes);
                }
                let arg1 = TreelandShortcutManagerV2BindError(arg1);
                if self.core.state.log {
                    let (millis, micros) = time_since_epoch();
                    let prefix = &self.core.state.log_prefix;
                    let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> treeland_shortcut_manager_v2#{}.commit_failure(name: {:?}, error: {:?})\n", msg[0], arg0, arg1);
                    self.core.state.log(args);
                }
                if let Some(handler) = handler {
                    (**handler).commit_failure(&self, arg0, arg1);
                } else {
                    DefaultHandler.commit_failure(&self, arg0, arg1);
                }
            }
            n => {
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError::UnknownMessageId(n));
            }
        }
        Ok(())
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "destroy",
            1 => "acquire",
            2 => "bind_key",
            3 => "bind_swipe_gesture",
            4 => "bind_hold_gesture",
            5 => "commit",
            6 => "unbind",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "activated",
            1 => "commit_success",
            2 => "commit_failure",
            _ => return None,
        };
        Some(name)
    }
}

impl Proxy for TreelandShortcutManagerV2 {
    fn core(&self) -> &ProxyCore {
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

impl TreelandShortcutManagerV2 {
    /// Since when the direction.down enum variant is available.
    pub const ENM__DIRECTION_DOWN__SINCE: u32 = 1;
    /// Since when the direction.left enum variant is available.
    pub const ENM__DIRECTION_LEFT__SINCE: u32 = 1;
    /// Since when the direction.up enum variant is available.
    pub const ENM__DIRECTION_UP__SINCE: u32 = 1;
    /// Since when the direction.right enum variant is available.
    pub const ENM__DIRECTION_RIGHT__SINCE: u32 = 1;

    /// Since when the action.notify enum variant is available.
    pub const ENM__ACTION_NOTIFY__SINCE: u32 = 1;
    /// Since when the action.workspace_1 enum variant is available.
    pub const ENM__ACTION_WORKSPACE_1__SINCE: u32 = 1;
    /// Since when the action.workspace_2 enum variant is available.
    pub const ENM__ACTION_WORKSPACE_2__SINCE: u32 = 1;
    /// Since when the action.workspace_3 enum variant is available.
    pub const ENM__ACTION_WORKSPACE_3__SINCE: u32 = 1;
    /// Since when the action.workspace_4 enum variant is available.
    pub const ENM__ACTION_WORKSPACE_4__SINCE: u32 = 1;
    /// Since when the action.workspace_5 enum variant is available.
    pub const ENM__ACTION_WORKSPACE_5__SINCE: u32 = 1;
    /// Since when the action.workspace_6 enum variant is available.
    pub const ENM__ACTION_WORKSPACE_6__SINCE: u32 = 1;
    /// Since when the action.prev_workspace enum variant is available.
    pub const ENM__ACTION_PREV_WORKSPACE__SINCE: u32 = 1;
    /// Since when the action.next_workspace enum variant is available.
    pub const ENM__ACTION_NEXT_WORKSPACE__SINCE: u32 = 1;
    /// Since when the action.show_desktop enum variant is available.
    pub const ENM__ACTION_SHOW_DESKTOP__SINCE: u32 = 1;
    /// Since when the action.maximize enum variant is available.
    pub const ENM__ACTION_MAXIMIZE__SINCE: u32 = 1;
    /// Since when the action.cancel_maximize enum variant is available.
    pub const ENM__ACTION_CANCEL_MAXIMIZE__SINCE: u32 = 1;
    /// Since when the action.move_window enum variant is available.
    pub const ENM__ACTION_MOVE_WINDOW__SINCE: u32 = 1;
    /// Since when the action.close_window enum variant is available.
    pub const ENM__ACTION_CLOSE_WINDOW__SINCE: u32 = 1;
    /// Since when the action.show_window_menu enum variant is available.
    pub const ENM__ACTION_SHOW_WINDOW_MENU__SINCE: u32 = 1;
    /// Since when the action.open_multitask_view enum variant is available.
    pub const ENM__ACTION_OPEN_MULTITASK_VIEW__SINCE: u32 = 1;
    /// Since when the action.close_multitask_view enum variant is available.
    pub const ENM__ACTION_CLOSE_MULTITASK_VIEW__SINCE: u32 = 1;
    /// Since when the action.toggle_multitask_view enum variant is available.
    pub const ENM__ACTION_TOGGLE_MULTITASK_VIEW__SINCE: u32 = 1;
    /// Since when the action.toggle_fps_display enum variant is available.
    pub const ENM__ACTION_TOGGLE_FPS_DISPLAY__SINCE: u32 = 1;
    /// Since when the action.lockscreen enum variant is available.
    pub const ENM__ACTION_LOCKSCREEN__SINCE: u32 = 1;
    /// Since when the action.shutdown_menu enum variant is available.
    pub const ENM__ACTION_SHUTDOWN_MENU__SINCE: u32 = 1;
    /// Since when the action.quit enum variant is available.
    pub const ENM__ACTION_QUIT__SINCE: u32 = 1;
    /// Since when the action.taskswitch_enter enum variant is available.
    pub const ENM__ACTION_TASKSWITCH_ENTER__SINCE: u32 = 1;
    /// Since when the action.taskswitch_next enum variant is available.
    pub const ENM__ACTION_TASKSWITCH_NEXT__SINCE: u32 = 1;
    /// Since when the action.taskswitch_prev enum variant is available.
    pub const ENM__ACTION_TASKSWITCH_PREV__SINCE: u32 = 1;
    /// Since when the action.taskswitch_sameapp_next enum variant is available.
    pub const ENM__ACTION_TASKSWITCH_SAMEAPP_NEXT__SINCE: u32 = 1;
    /// Since when the action.taskswitch_sameapp_prev enum variant is available.
    pub const ENM__ACTION_TASKSWITCH_SAMEAPP_PREV__SINCE: u32 = 1;

    /// Since when the keybind_mode.key_release enum variant is available.
    pub const ENM__KEYBIND_MODE_KEY_RELEASE__SINCE: u32 = 1;
    /// Since when the keybind_mode.key_press enum variant is available.
    pub const ENM__KEYBIND_MODE_KEY_PRESS__SINCE: u32 = 1;
    /// Since when the keybind_mode.key_press_repeat enum variant is available.
    pub const ENM__KEYBIND_MODE_KEY_PRESS_REPEAT__SINCE: u32 = 1;

    /// Since when the bind_error.name_conflict enum variant is available.
    pub const ENM__BIND_ERROR_NAME_CONFLICT__SINCE: u32 = 1;
    /// Since when the bind_error.duplicate_binding enum variant is available.
    pub const ENM__BIND_ERROR_DUPLICATE_BINDING__SINCE: u32 = 1;
    /// Since when the bind_error.invalid_argument enum variant is available.
    pub const ENM__BIND_ERROR_INVALID_ARGUMENT__SINCE: u32 = 1;
    /// Since when the bind_error.internal_error enum variant is available.
    pub const ENM__BIND_ERROR_INTERNAL_ERROR__SINCE: u32 = 1;

    /// Since when the error.occupied enum variant is available.
    pub const ENM__ERROR_OCCUPIED__SINCE: u32 = 1;
    /// Since when the error.not_acquired enum variant is available.
    pub const ENM__ERROR_NOT_ACQUIRED__SINCE: u32 = 1;
    /// Since when the error.invalid_commit enum variant is available.
    pub const ENM__ERROR_INVALID_COMMIT__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TreelandShortcutManagerV2Direction(pub u32);

impl TreelandShortcutManagerV2Direction {
    pub const DOWN: Self = Self(1);

    pub const LEFT: Self = Self(2);

    pub const UP: Self = Self(3);

    pub const RIGHT: Self = Self(4);
}

impl Debug for TreelandShortcutManagerV2Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::DOWN => "DOWN",
            Self::LEFT => "LEFT",
            Self::UP => "UP",
            Self::RIGHT => "RIGHT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// compositor actions
///
/// Compositor actions that can be assigned to a shortcut.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TreelandShortcutManagerV2Action(pub u32);

impl TreelandShortcutManagerV2Action {
    pub const NOTIFY: Self = Self(1);

    pub const WORKSPACE_1: Self = Self(2);

    pub const WORKSPACE_2: Self = Self(3);

    pub const WORKSPACE_3: Self = Self(4);

    pub const WORKSPACE_4: Self = Self(5);

    pub const WORKSPACE_5: Self = Self(6);

    pub const WORKSPACE_6: Self = Self(7);

    pub const PREV_WORKSPACE: Self = Self(8);

    pub const NEXT_WORKSPACE: Self = Self(9);

    pub const SHOW_DESKTOP: Self = Self(10);

    pub const MAXIMIZE: Self = Self(11);

    pub const CANCEL_MAXIMIZE: Self = Self(12);

    pub const MOVE_WINDOW: Self = Self(13);

    pub const CLOSE_WINDOW: Self = Self(14);

    pub const SHOW_WINDOW_MENU: Self = Self(15);

    pub const OPEN_MULTITASK_VIEW: Self = Self(16);

    pub const CLOSE_MULTITASK_VIEW: Self = Self(17);

    pub const TOGGLE_MULTITASK_VIEW: Self = Self(18);

    pub const TOGGLE_FPS_DISPLAY: Self = Self(19);

    pub const LOCKSCREEN: Self = Self(20);

    pub const SHUTDOWN_MENU: Self = Self(21);

    pub const QUIT: Self = Self(22);

    pub const TASKSWITCH_ENTER: Self = Self(23);

    pub const TASKSWITCH_NEXT: Self = Self(24);

    pub const TASKSWITCH_PREV: Self = Self(25);

    pub const TASKSWITCH_SAMEAPP_NEXT: Self = Self(26);

    pub const TASKSWITCH_SAMEAPP_PREV: Self = Self(27);
}

impl Debug for TreelandShortcutManagerV2Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NOTIFY => "NOTIFY",
            Self::WORKSPACE_1 => "WORKSPACE_1",
            Self::WORKSPACE_2 => "WORKSPACE_2",
            Self::WORKSPACE_3 => "WORKSPACE_3",
            Self::WORKSPACE_4 => "WORKSPACE_4",
            Self::WORKSPACE_5 => "WORKSPACE_5",
            Self::WORKSPACE_6 => "WORKSPACE_6",
            Self::PREV_WORKSPACE => "PREV_WORKSPACE",
            Self::NEXT_WORKSPACE => "NEXT_WORKSPACE",
            Self::SHOW_DESKTOP => "SHOW_DESKTOP",
            Self::MAXIMIZE => "MAXIMIZE",
            Self::CANCEL_MAXIMIZE => "CANCEL_MAXIMIZE",
            Self::MOVE_WINDOW => "MOVE_WINDOW",
            Self::CLOSE_WINDOW => "CLOSE_WINDOW",
            Self::SHOW_WINDOW_MENU => "SHOW_WINDOW_MENU",
            Self::OPEN_MULTITASK_VIEW => "OPEN_MULTITASK_VIEW",
            Self::CLOSE_MULTITASK_VIEW => "CLOSE_MULTITASK_VIEW",
            Self::TOGGLE_MULTITASK_VIEW => "TOGGLE_MULTITASK_VIEW",
            Self::TOGGLE_FPS_DISPLAY => "TOGGLE_FPS_DISPLAY",
            Self::LOCKSCREEN => "LOCKSCREEN",
            Self::SHUTDOWN_MENU => "SHUTDOWN_MENU",
            Self::QUIT => "QUIT",
            Self::TASKSWITCH_ENTER => "TASKSWITCH_ENTER",
            Self::TASKSWITCH_NEXT => "TASKSWITCH_NEXT",
            Self::TASKSWITCH_PREV => "TASKSWITCH_PREV",
            Self::TASKSWITCH_SAMEAPP_NEXT => "TASKSWITCH_SAMEAPP_NEXT",
            Self::TASKSWITCH_SAMEAPP_PREV => "TASKSWITCH_SAMEAPP_PREV",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// keybinding modes
///
/// Keybinding modes.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TreelandShortcutManagerV2KeybindMode(pub u32);

impl TreelandShortcutManagerV2KeybindMode {
    pub const KEY_RELEASE: Self = Self(1);

    pub const KEY_PRESS: Self = Self(2);

    pub const KEY_PRESS_REPEAT: Self = Self(3);
}

impl Debug for TreelandShortcutManagerV2KeybindMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::KEY_RELEASE => "KEY_RELEASE",
            Self::KEY_PRESS => "KEY_PRESS",
            Self::KEY_PRESS_REPEAT => "KEY_PRESS_REPEAT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// binding error codes
///
/// Error codes indicating the reason of a binding failure.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TreelandShortcutManagerV2BindError(pub u32);

impl TreelandShortcutManagerV2BindError {
    pub const NAME_CONFLICT: Self = Self(1);

    pub const DUPLICATE_BINDING: Self = Self(2);

    pub const INVALID_ARGUMENT: Self = Self(3);

    pub const INTERNAL_ERROR: Self = Self(4);
}

impl Debug for TreelandShortcutManagerV2BindError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NAME_CONFLICT => "NAME_CONFLICT",
            Self::DUPLICATE_BINDING => "DUPLICATE_BINDING",
            Self::INVALID_ARGUMENT => "INVALID_ARGUMENT",
            Self::INTERNAL_ERROR => "INTERNAL_ERROR",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TreelandShortcutManagerV2Error(pub u32);

impl TreelandShortcutManagerV2Error {
    pub const OCCUPIED: Self = Self(1);

    pub const NOT_ACQUIRED: Self = Self(2);

    pub const INVALID_COMMIT: Self = Self(3);
}

impl Debug for TreelandShortcutManagerV2Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::OCCUPIED => "OCCUPIED",
            Self::NOT_ACQUIRED => "NOT_ACQUIRED",
            Self::INVALID_COMMIT => "INVALID_COMMIT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
