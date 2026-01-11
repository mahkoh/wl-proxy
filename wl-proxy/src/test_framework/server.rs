use {
    crate::{
        baseline::Baseline,
        client::ClientHandler,
        global_mapper::GlobalMapper,
        object::{Object, ObjectCoreApi, ObjectRcUtils},
        protocols::{
            ObjectInterface,
            ext_workspace_v1::ext_workspace_handle_v1::ExtWorkspaceHandleV1,
            wayland::{
                wl_buffer::{WlBuffer, WlBufferHandler},
                wl_callback::WlCallback,
                wl_display::{WlDisplay, WlDisplayHandler},
                wl_registry::{WlRegistry, WlRegistryHandler},
                wl_shm::{WlShm, WlShmFormat, WlShmHandler},
                wl_shm_pool::{WlShmPool, WlShmPoolHandler},
            },
        },
        state::State,
    },
    std::{os::fd::OwnedFd, rc::Rc, sync::mpsc, thread},
};

pub fn test_server(log: bool) -> Rc<OwnedFd> {
    let (send, recv) = mpsc::channel();
    thread::spawn(move || {
        let state = State::builder(Baseline::ALL_OF_THEM)
            .without_server()
            .with_logging(log)
            .with_log_prefix("server")
            .build()
            .unwrap();
        state.set_default_forward_to_server(false);
        let (client, fd) = state.connect().unwrap();
        send.send(fd).unwrap();
        client.set_handler(ClientHandlerImpl {
            state: state.clone(),
        });
        client.display().set_handler(DisplayHandler {});
        while state.is_not_destroyed() {
            state.dispatch_blocking().unwrap();
        }
    });
    Rc::new(recv.recv().unwrap())
}

struct ClientHandlerImpl {
    state: Rc<State>,
}

impl ClientHandler for ClientHandlerImpl {
    fn disconnected(self: Box<Self>) {
        self.state.destroy();
    }
}

struct DisplayHandler {}

impl WlDisplayHandler for DisplayHandler {
    fn handle_sync(&mut self, _slf: &Rc<WlDisplay>, callback: &Rc<WlCallback>) {
        callback.send_done(0);
        callback.delete_id();
    }

    fn handle_get_registry(&mut self, _slf: &Rc<WlDisplay>, registry: &Rc<WlRegistry>) {
        let mut mapper = GlobalMapper::default();
        registry.set_handler(RegistryHandler {
            wl_shm_name: mapper.add_synthetic_global(registry, ObjectInterface::WlShm, 2),
            ext_workspace_handle_name: mapper.add_synthetic_global(
                registry,
                ObjectInterface::ExtWorkspaceHandleV1,
                1,
            ),
            _mapper: mapper,
        });
    }
}

struct RegistryHandler {
    _mapper: GlobalMapper,
    wl_shm_name: u32,
    ext_workspace_handle_name: u32,
}

impl WlRegistryHandler for RegistryHandler {
    fn handle_bind(&mut self, _slf: &Rc<WlRegistry>, name: u32, id: Rc<dyn Object>) {
        if name == self.wl_shm_name {
            let id = id.downcast::<WlShm>();
            id.set_handler(ShmHandler {});
            id.send_format(WlShmFormat::ARGB8888);
            id.send_format(WlShmFormat::XRGB8888);
        } else if name == self.ext_workspace_handle_name {
            let id = id.downcast::<ExtWorkspaceHandleV1>();
            id.send_coordinates(&[1]);
        }
    }
}

struct ShmHandler {}

impl WlShmHandler for ShmHandler {
    fn handle_create_pool(
        &mut self,
        _slf: &Rc<WlShm>,
        id: &Rc<WlShmPool>,
        _fd: &Rc<OwnedFd>,
        _size: i32,
    ) {
        id.set_handler(ShmPoolHandler {});
    }

    fn handle_release(&mut self, slf: &Rc<WlShm>) {
        slf.delete_id();
    }
}

struct ShmPoolHandler {}

impl WlShmPoolHandler for ShmPoolHandler {
    fn handle_create_buffer(
        &mut self,
        _slf: &Rc<WlShmPool>,
        id: &Rc<WlBuffer>,
        _offset: i32,
        _width: i32,
        _height: i32,
        _stride: i32,
        _format: WlShmFormat,
    ) {
        id.set_handler(BufferHandler {});
    }

    fn handle_destroy(&mut self, slf: &Rc<WlShmPool>) {
        slf.delete_id();
    }

    fn handle_resize(&mut self, _slf: &Rc<WlShmPool>, _size: i32) {
        // nothing
    }
}

struct BufferHandler {}

impl WlBufferHandler for BufferHandler {
    fn handle_destroy(&mut self, slf: &Rc<WlBuffer>) {
        slf.delete_id();
    }
}
