#![expect(
    clippy::single_char_add_str,
    clippy::manual_is_multiple_of,
    clippy::manual_div_ceil
)]

//! This crate can be used to proxy wayland connections and manipulate wayland messages.
//!
//! # Example
//!
//! This example spawns mpv and hides the wp_fifo_manager_v1 global from it.
//!
//! ```no_run
//! use std::process::Command;
//! use std::rc::Rc;
//! use wl_proxy::baseline::Baseline;
//! use wl_proxy::global_mapper::GlobalMapper;
//! use wl_proxy::object::Object;
//! use wl_proxy::protocols::ObjectInterface;
//! use wl_proxy::protocols::wayland::wl_display::{WlDisplay, WlDisplayHandler};
//! use wl_proxy::protocols::wayland::wl_region::WlRegionHandler;
//! use wl_proxy::protocols::wayland::wl_registry::{WlRegistry, WlRegistryHandler};
//! use wl_proxy::simple::{SimpleProxy, SimpleCommandExt};
//!
//! // SimpleProxy is a helper type that creates one proxy per client and runs the proxy
//! // in a thread.
//! // Baselines are used to limit the protocols exposed by the proxy. This allows
//! // semver-compatible updates of this crate to add new protocols and protocol versions
//! // without causing protocol errors in existing proxies. Since this proxy is very
//! // simple, we just expose all protocols supported by the crate.
//! let proxy = SimpleProxy::new(Baseline::ALL_OF_THEM).unwrap();
//! // This starts mpv and spawns a thread that waits for mpv to exit. When mpv exits,
//! // the thread also calls exit to forward the exit code.
//! Command::new("mpv")
//!     .with_wayland_display(proxy.display())
//!     .spawn_and_forward_exit_code()
//!     .unwrap();
//! // The closure is invoked once for each client that connects and must return a
//! // WlDisplayHandler.
//! // This function does not return unless there is a fatal error.
//! proxy.run(|| Display);
//!
//! struct Display;
//!
//! impl WlDisplayHandler for Display {
//!     // This function is invoked when the client sends a get_registry request.
//!     fn handle_get_registry(&mut self, slf: &Rc<WlDisplay>, registry: &Rc<WlRegistry>) {
//!         // Forward the request to the compositor.
//!         slf.send_get_registry(registry);
//!         // Install a message handler for the registry.
//!         registry.set_handler(Registry::default());
//!     }
//! }
//!
//! #[derive(Default)]
//! struct Registry {
//!     // This helper allows the proxy to easily filter compositor globals and to
//!     // announce its own globals.
//!     filter: GlobalMapper,
//! }
//!
//! impl WlRegistryHandler for Registry {
//!     // This function is invoked when the client sends a bind request.
//!     fn handle_bind(&mut self, slf: &Rc<WlRegistry>, name: u32, id: Rc<dyn Object>) {
//!         // This function forwards the bind request to the compositor.
//!         self.filter.forward_bind(slf, name, &id);
//!     }
//!
//!     // This function is invoked when the compositor sends a global event.
//!     fn handle_global(&mut self, slf: &Rc<WlRegistry>, name: u32, interface: ObjectInterface, version: u32) {
//!         if interface == ObjectInterface::WpFifoManagerV1 {
//!             // This function does not forward the global to the client and marks it
//!             // as ignored so that the corresponding global_remove event can be
//!             // filtered as well.
//!             self.filter.ignore_global(name);
//!         } else {
//!             // This function forwards the global to the client.
//!             self.filter.forward_global(slf, name, interface, version);
//!         }
//!     }
//!
//!     fn handle_global_remove(&mut self, slf: &Rc<WlRegistry>, name: u32) {
//!         self.filter.forward_global_remove(slf, name);
//!     }
//! }
//! ```
//!
//! A more complex proxy might not use [`SimpleProxy`](simple::SimpleProxy) and could
//! instead construct [`State`](state::State) objects manually. These objects can be
//! polled with an external mechanism such as epoll and therefore integrate into existing
//! event loops. They allow multiple client connections to be proxied over the same
//! compositor connection, so that they can share wayland objects. This can be used, for
//! example, to embed one client into another.
//!
//! The crate repository contains several more complex examples.
//!
//! # Objects
//!
//! The [`Object`](object::Object) trait represents wayland objects such as wl_display or
//! wl_registry.
//!
//! Objects are created in one of three ways:
//!
//! - The client sends a request with a new_id parameter.
//! - The compositor sends an event with a new_id parameter.
//! - The proxy itself creates the object.
//!
//! The case where the client creates an object with a request can be seen above in the
//! wl_display.get_registry and wl_registry.bind handlers.
//!
//! Each object
//!
//! - can be associated with 0 or 1 objects in the compositor and
//! - can be associated with 0 or 1 objects in a client.
//!
//! Objects created by the client start out as associated with the client but not with
//! the compositor. Objects created by the compositor start out as associated with the
//! compositor but not with any client. Objects created by the proxy start out
//! unassociated.
//!
//! Associations are created implicitly. For example,
//!
//! ```no_run
//! # use std::rc::Rc;
//! # use wl_proxy::protocols::wayland::wl_display::{WlDisplay, WlDisplayHandler};
//! # use wl_proxy::protocols::wayland::wl_registry::WlRegistry;
//! #
//! struct Display;
//!
//! impl WlDisplayHandler for Display {
//!     fn handle_get_registry(&mut self, slf: &Rc<WlDisplay>, registry: &Rc<WlRegistry>) {
//!         // At this point the WlRegistry is associated with a client-side wl_registry
//!         // but not yet with a compositor object.
//!
//!         // By forwarding the wl_display.get_registry request to the compositor, we
//!         // create a new wl_registry object in the compositor and associate the
//!         // WlRegistry with it.
//!         slf.send_get_registry(registry);
//!     }
//! }
//! ```
//!
//! An object becomes unassociated through destructor messages or through delete_id
//! messages, if delete_id messages are used for the object. Once an object is
//! unassociated, it can once again become associated if it is used in a subsequent
//! constructor request or event.
//!
//! It is valid for an object to never be associated with a compositor or client object.
//! For example, this can happen if a client-created object is handled internally by the
//! proxy without ever forwarding it to the compositor.
//!
//! Each object can only be associated with one client at a time. Multiplexing must be
//! implemented manually.
//!
//! # Handlers
//!
//! Each object can have a handler that implements the corresponding trait. For example,
//! [`WlRegistryHandler`](protocols::wayland::wl_registry::WlRegistryHandler).
//!
//! All functions in a handler have default implementations that try to forward the
//! message to the peer. This behavior can be adjusted by using
//! [`ObjectCoreApi::set_forward_to_client`](object::ObjectCoreApi::set_forward_to_client)
//! and
//! [`ObjectCoreApi::set_forward_to_server`](object::ObjectCoreApi::set_forward_to_server).
//!
//! If an object does not have a handler, this default behavior is used for all messages.
//!
//! All handler functions use a `&mut self` receiver. The handlers of other objects can
//! be accessed with [`ObjectUtils::get_handler_ref`](object::ObjectUtils::get_handler_ref)
//! and [`ObjectUtils::get_handler_mut`](object::ObjectUtils::get_handler_mut). These
//! functions panic if the handler already has an incompatible borrow or if an invalid
//! type is used.
//!
//! ```no_run
//! # use std::rc::Rc;
//! # use wl_proxy::object::ObjectUtils;
//! # use wl_proxy::protocols::wayland::wl_compositor::{WlCompositor, WlCompositorHandler};
//! # use wl_proxy::protocols::wayland::wl_surface::{WlSurface, WlSurfaceHandler};
//! # use wl_proxy::protocols::xdg_shell::xdg_surface::XdgSurface;
//! # use wl_proxy::protocols::xdg_shell::xdg_wm_base::{XdgWmBase, XdgWmBaseHandler};
//! #
//! struct Compositor;
//!
//! impl WlCompositorHandler for Compositor {
//!     fn handle_create_surface(&mut self, slf: &Rc<WlCompositor>, id: &Rc<WlSurface>) {
//!         slf.send_create_surface(id);
//!         // set this handler for all wl_surface objects created by the client
//!         id.set_handler(Surface);
//!     }
//! }
//!
//! struct Surface;
//!
//! impl WlSurfaceHandler for Surface { }
//!
//! struct WmBase;
//!
//! impl XdgWmBaseHandler for WmBase {
//!     fn handle_get_xdg_surface(&mut self, _slf: &Rc<XdgWmBase>, id: &Rc<XdgSurface>, surface: &Rc<WlSurface>) {
//!         // ok:
//!         // - we always set this handler type
//!         // - since we're in an XdgWmBaseHandler, only the handler of the XdgWmBase is
//!         //   currently borrowed
//!         let surface_handler = &mut *surface.get_handler_mut::<Surface>();
//!     }
//! }
//! ```
//!
//! The `try_get_handler_ref` and `try_get_handler_mut` functions can be used to handle
//! such errors gracefully.
//!
//! Handlers can be unset by calling [`Object::unset_handler`](object::Object::unset_handler).
//! Both `set_handler` and `unset_handler` can be called at any time, even if the object
//! is currently borrowed. If the handler is borrowed, the change will happen at the next
//! possible moment.
//!
//! Handlers are unset implicitly if the [`State`](state::State) is destroyed. If a
//! handler forms a reference cycle that would prevent the containing object from reaching
//! a reference count of 0, the handler must be unset manually if it is supposed to be
//! dropped before the state is destroyed. Most commonly, the handler should be unset
//! when the object is logically destroyed.
//!
//! # Sending Messages
//!
//! Objects define functions that can be used to send messages. Whether the message is
//! sent to the compositor or the client depends implicitly on the type of the message.
//!
//! For each message there are usually two functions: `send_*` and `try_send_*`. For
//! example,
//! [`WlDisplay::send_get_registry`](protocols::wayland::wl_display::WlDisplay::send_get_registry)
//! and
//! [`WlDisplay::try_send_get_registry`](protocols::wayland::wl_display::WlDisplay::try_send_get_registry).
//! The difference is that `send_*` logs errors instead of returning them. Errors in these
//! functions usually occur because the object is not associated with the compositor or
//! a client or if one of the object arguments is not associated with the compositor or
//! the client.
//!
//! For messages with new_id parameters, there are two additional functions: `new_send_*`,
//! and `new_try_send_*`. For example,
//! [`WlDisplay::new_send_get_registry`](protocols::wayland::wl_display::WlDisplay::new_send_get_registry)
//! and
//! [`WlDisplay::new_try_send_get_registry`](protocols::wayland::wl_display::WlDisplay::new_try_send_get_registry).
//! Instead of taking the new_id parameter as an argument, they create a new object and
//! return it.
//!
//! # delete_id message
//!
//! If an object is associated with the compositor, the association was created by the
//! proxy (i.e. not via a new_id _event_), and the association is subsequently removed, the
//! compositor will send a wl_display.delete_id message with the object ID. The proxy can
//! intercept this message by overriding the delete_id function in the object handler. By
//! default, the delete_id message is forwarded to the client.
//!
//! In an object is associated with a client, the association was created by the client
//! with a request containing a new_id parameter, and the association is subsequently
//! removed, the proxy _must_ send a wl_display.delete_id message with the object ID. If
//! the object is also associated with the compositor, then this might happen
//! automatically as described in the previous paragraph. Otherwise, the proxy has to
//! manually invoke the [`ObjectCoreApi::delete_id`](object::ObjectCoreApi::delete_id)
//! function on the object. For example, if the object is handled inside the proxy without
//! involving the compositor, the proxy might invoke this function at the end of a
//! destructor request.
//!
//! ```
//! # use std::rc::Rc;
//! # use wl_proxy::object::ObjectCoreApi;
//! # use wl_proxy::protocols::wayland::wl_surface::{WlSurface, WlSurfaceHandler};
//! #
//! struct Surface;
//!
//! impl WlSurfaceHandler for Surface {
//!     fn handle_destroy(&mut self, slf: &Rc<WlSurface>) {
//!         slf.delete_id();
//!     }
//! }
//! ```
//!
//! # Protocol Support
//!
//! This crate supports most of the protocols from [wayland-db](https://github.com/mahkoh/wayland-db).
//!
//! Protocol XML files are generated from the database by running the update-protocols
//! application in the wl-proxy repository. Rust code is generated from these XML files by
//! running the generator application. If an application needs to support a protocol that
//! is not currently supported by wl-proxy, the application can vendor the wl-proxy
//! repository, add the protocol XML file, and re-run the generator.
//!
//! Some protocols had to be excluded because they don't use proper namespacing. This
//! mostly affects protocols from the plasma-wayland-protocols suite.
//!
//! Protocols that use the same message name multiple times in the same interface are
//! currently not supported. This currently only affects the `cosmic_workspace_unstable_v1`
//! protocol and its dependents. If there is demand, this can be worked around by
//! renaming these conflicting messages within update-protocols.
//!
//! The core wayland protocol is always available. All other protocols must be enabled via
//! cargo features. If a protocol depends on another protocol, the dependency is enabled
//! automatically. There are also cargo features named `suite-*` that can be used to
//! enable all protocols from specific suites.
//!
//! # Synchronizing With Clients
//!
//! In some situations it might be desirable for a client to synchronize messages with
//! the proxy. For example, consider the following three actors:
//!
//! - The proxy
//! - An embedding client A
//! - An embedded client B
//!
//! Both A and B are proxied over the same compositor connection, allowing A to embed
//! surfaces from B into its own application. The proxy runs in the same process as
//! client A.
//!
//! It might be necessary for the process to match objects created by the wayland client
//! of A with proxy objects. This can be accomplished by having A or the proxy send
//! synchronization messages.
//!
//! This [wlproxy_sync_v1](https://github.com/mahkoh/wl-proxy/blob/master/protocols/wlproxy/wlproxy-sync-v1.xml)
//! protocol can be used for this.
//!
//! # Logging
//!
//! Messages sent in all directions can be logged. The logged messages look like this:
//!
//! ```text
//! [1679788.330] {socket-1} client#2    -> wl_display#1.get_registry(registry: wl_registry#2)
//! ^~~~~~~~~~~~~
//!  timestamp of the message
//!               ^~~~~~~~~~
//!                the logging prefix of the State
//!                          ^~~~~~~~
//!                           the sender or receiver
//!                                      ^~
//!                                       the direction of the message
//!                                         ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//!                                          the message
//! ```
//!
//! By default, logging is controlled by the `WL_PROXY_DEBUG` environment variable. If it
//! is set to 1, messages are logged, otherwise messages are not logged. Messages are
//! always written to STDERR.
//!
//! Applications can disable or enable logging programmatically via
//! [`StateBuilder::with_logging`](state::StateBuilder::with_logging).
//!
//! Messages can have a prefix that is taken from the `WL_PROXY_PREFIX` environment
//! variable. Applications can augment this prefix via
//! [`StateBuilder::with_log_prefix`](state::StateBuilder::with_log_prefix). This can be
//! used to easily distinguish these log messages from WAYLAND_DEBUG messages. The
//! [`SimpleProxy`](simple::SimpleProxy) always sets the prefix to a unique identifier
//! of the connection to allow distinguishing messages from different clients.
//!
//! Each message indicates the sender or receiver of the message. For messages sent by/to
//! clients, the client is identified as `client#N`. Note that `N` does not start at 1 nor
//! are the number contiguous. For messages sent by/to the compositor, the identifier is
//! `server`.
//!
//! The symbols `->` and `<=` indicate the direction of the message.
//!
//! Logging can be disabled at compile time by disabling the default-enabled `logging`
//! feature. This can significantly improve compile times and reduce binary size. For
//! example, the wl-veil application has a program size of 6.7 MB by default but only
//! 4.4 MB with logging disabled at compile time. Most programs should leave logging
//! enabled to aid debuggability.

pub mod acceptor;
pub mod client;
mod endpoint;
pub mod fixed;
mod protocol_helpers;
/// Auto-generated wayland protocols.
#[rustfmt::skip]
pub mod protocols;
pub mod baseline;
pub mod global_mapper;
pub mod handler;
pub mod object;
mod poll;
pub mod simple;
pub mod state;
#[cfg(test)]
mod test_framework;
mod trans;
mod utils;
mod wire;
