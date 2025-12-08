use {
    crate::{
        client::Client,
        free_list::FreeList,
        object_error::ObjectError,
        proxy::Proxy,
        trans::{self, FlushResult, InputBuffer, OutputSwapchain, TransError},
    },
    std::{
        cell::{Cell, RefCell},
        collections::{HashMap, VecDeque},
        os::fd::{AsRawFd, OwnedFd},
        rc::Rc,
    },
    thiserror::Error,
    uapi::c,
};

pub struct Endpoint {
    pub id: u64,
    pub socket: Rc<OwnedFd>,
    pub outgoing: RefCell<OutputSwapchain>,
    pub has_outgoing: Cell<bool>,
    pub objects: RefCell<HashMap<u32, Rc<dyn Proxy>>>,
    pub idl: FreeList<u32, 3>,
    pub current_interest: Cell<c::c_int>,
    pub desired_interest: Cell<c::c_int>,
    pub interest_update_queued: Cell<bool>,
    pub incoming: RefCell<InputState>,
}

#[derive(Default)]
pub struct InputState {
    pub buffer: Box<InputBuffer>,
    pub fds: VecDeque<Rc<OwnedFd>>,
}

#[derive(Debug, Error)]
pub enum EndpointError {
    #[error("could not flush the socket")]
    Flush(#[source] TransError),
    #[error("could not read a message")]
    Read(#[source] TransError),
    #[error("receiver object {} does not exist", .0)]
    NoReceiver(u32),
    #[error("could not handle a message")]
    HandleMessage(#[source] ObjectError),
}

impl Endpoint {
    pub fn new(id: u64, socket: OwnedFd) -> Rc<Self> {
        Rc::new(Endpoint {
            id,
            socket: Rc::new(socket),
            outgoing: Default::default(),
            has_outgoing: Default::default(),
            objects: Default::default(),
            idl: Default::default(),
            current_interest: Default::default(),
            desired_interest: Default::default(),
            interest_update_queued: Default::default(),
            incoming: Default::default(),
        })
    }

    pub fn lookup(&self, id: u32) -> Option<Rc<dyn Proxy>> {
        self.objects.borrow().get(&id).cloned()
    }

    pub fn flush(&self) -> Result<FlushResult, EndpointError> {
        self.outgoing
            .borrow_mut()
            .flush(self.socket.as_raw_fd())
            .map_err(EndpointError::Flush)
    }

    pub fn dispatch(&self, client: Option<&Rc<Client>>) -> Result<(), EndpointError> {
        let incoming = &mut *self.incoming.borrow_mut();
        let buffer = &mut *incoming.buffer;
        let fds = &mut incoming.fds;
        let mut may_read_from_socket = true;
        loop {
            let msg = trans::read_message(
                self.socket.as_raw_fd(),
                &mut may_read_from_socket,
                buffer,
                fds,
            );
            let Some(msg) = msg.map_err(EndpointError::Read)? else {
                break;
            };
            let obj = msg[0];
            let obj = self
                .objects
                .borrow()
                .get(&obj)
                .cloned()
                .ok_or(EndpointError::NoReceiver(obj))?;
            let res = if let Some(client) = client {
                eprintln!(
                    "client {}: -> {}.{}",
                    client.endpoint.id,
                    msg[0],
                    msg[1] & 0xffff
                );
                obj.handle_request(client, msg, fds)
            } else {
                eprintln!("server   : -> {}.{}", msg[0], msg[1] & 0xffff);
                obj.handle_event(msg, fds)
            };
            res.map_err(EndpointError::HandleMessage)?;
        }
        Ok(())
    }
}
