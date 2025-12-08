pub(crate) struct Protocol {
    pub(crate) name: String,
    pub(crate) is_wayland: bool,
    pub(crate) is_wlproxy_test: bool,
    pub(crate) _copyright: Option<Copyright>,
    pub(crate) description: Option<Description>,
    pub(crate) interfaces: Vec<Interface>,
}

pub(crate) struct Copyright {
    pub(crate) _body: String,
}

#[derive(Debug)]
pub(crate) struct Description {
    pub(crate) summary: Option<String>,
    pub(crate) body: String,
}

pub(crate) struct Interface {
    pub(crate) name: String,
    pub(crate) version: u32,
    pub(crate) is_wl_display: bool,
    pub(crate) is_wl_registry: bool,
    pub(crate) is_wl_fixes: bool,
    pub(crate) description: Option<Description>,
    pub(crate) messages: Vec<Message>,
    pub(crate) enums: Vec<Enum>,
}

#[derive(Debug)]
pub(crate) struct Arg {
    pub(crate) name: String,
    pub(crate) ty: ArgType,
    pub(crate) summary: Option<String>,
    pub(crate) description: Option<Description>,
    pub(crate) interface: Option<String>,
    pub(crate) allow_null: bool,
    pub(crate) enum_: Option<String>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum ArgType {
    NewId,
    Int,
    Uint,
    Fixed,
    String,
    Object,
    Array,
    Fd,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum MessageType {
    Destructor,
}

pub(crate) struct Entry {
    pub(crate) name: String,
    pub(crate) value: String,
    pub(crate) value_u32: u32,
    pub(crate) summary: Option<String>,
    pub(crate) since: Option<u32>,
    pub(crate) deprecated_since: Option<u32>,
    pub(crate) description: Option<Description>,
}

pub(crate) struct Enum {
    pub(crate) name: String,
    pub(crate) _since: Option<u32>,
    pub(crate) bitfield: bool,
    pub(crate) description: Option<Description>,
    pub(crate) entries: Vec<Entry>,
}

pub(crate) struct Message {
    pub(crate) name: String,
    pub(crate) message_id: usize,
    pub(crate) is_request: bool,
    pub(crate) ty: Option<MessageType>,
    pub(crate) since: Option<u32>,
    pub(crate) deprecated_since: Option<u32>,
    pub(crate) description: Option<Description>,
    pub(crate) args: Vec<Arg>,
}
