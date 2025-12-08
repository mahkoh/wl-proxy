use {
    crate::ast::{Arg, ArgType, Description, Interface, Message, MessageType, Protocol},
    debug_fn::debug_fn,
    phf::phf_set,
    std::{
        fmt::{Display, Write as FmtWrite},
        io::{self, Write},
    },
};

macro_rules! define_w {
    ($w:ident) => {
        define_w!($w, wl, $);
    };
    ($w:ident, $wl:ident) => {
        define_w!($w, $wl, $);
    };
    ($w:ident, $wl:ident, $dol:tt) => {
        #[allow(unused_macros)]
        macro_rules! w {
            ($dol($arg:tt)*) => {
                write!($w, $dol($arg)*)
            };
        }
        macro_rules! $wl {
            ($dol($arg:tt)*) => {
                writeln!($w, $dol($arg)*)
            };
        }
    };
}

fn format_interface_header(w: &mut impl Write, interface: &Interface) -> io::Result<()> {
    define_w!(w);
    if let Some(desc) = &interface.description {
        format_description(w, "//!", desc)?;
        wl!()?;
    }
    wl!("use crate::generated_helper::prelude::*;")?;
    wl!("use super::super::all_types::*;")?;
    Ok(())
}

pub fn format_interface_file(
    w: &mut impl Write,
    mutable_data: bool,
    interface: &Interface,
) -> io::Result<()> {
    define_w!(w);
    format_interface_header(w, interface)?;
    wl!()?;
    format_interface_types(w, interface)?;
    wl!()?;
    format_interface_constructors(w, interface)?;
    wl!()?;
    format_interface_trait_impls(w, interface)?;
    wl!()?;
    format_interface_message_functions(w, interface)?;
    wl!()?;
    format_interface_message_handler(w, mutable_data, interface)?;
    wl!()?;
    format_proxy_impl(w, interface)?;
    wl!()?;
    format_interface_enums(w, interface)?;
    Ok(())
}

fn format_camel(s: &str) -> impl Display + use<'_> {
    debug_fn(move |f| {
        let mut need_uppercase = true;
        for &c in s.as_bytes() {
            if c == b'_' || c == b'.' {
                need_uppercase = true;
            } else if need_uppercase {
                need_uppercase = false;
                f.write_char(c.to_ascii_uppercase() as _)?;
            } else {
                f.write_char(c as _)?;
            }
        }
        Ok(())
    })
}

fn format_uppercase(s: &str) -> impl Display + use<'_> {
    debug_fn(move |f| {
        for &c in s.as_bytes() {
            f.write_char(c.to_ascii_uppercase() as _)?;
        }
        Ok(())
    })
}

fn format_enum_variant(s: &str) -> impl Display + use<'_> {
    let need_underscore = s.chars().next().unwrap_or_default().is_ascii_digit();
    debug_fn(move |f| {
        if need_underscore {
            f.write_str("_")?;
        }
        format_uppercase(s).fmt(f)
    })
}

fn format_interface_types(w: &mut impl Write, interface: &Interface) -> io::Result<()> {
    define_w!(w);
    let snake = &interface.name;
    let camel = format_camel(snake).to_string();
    wl!(r#"/// A {snake} proxy."#)?;
    wl!(r#"///"#)?;
    wl!(r#"/// See the documentation of [the module][self] for the interface description."#)?;
    wl!(r#"pub struct Meta{camel} {{"#)?;
    wl!(r#"    core: ProxyCore,"#)?;
    wl!(r#"    handler: MessageHandlerHolder<dyn Meta{camel}MessageHandler>,"#)?;
    wl!(r#"}}"#)?;
    if interface.messages.len() > 0 {
        wl!()?;
        wl!(r#"struct DefaultMessageHandler;"#)?;
        wl!()?;
        wl!(r#"impl Meta{camel}MessageHandler for DefaultMessageHandler {{ }}"#)?;
    }
    wl!()?;
    wl!(r#"impl Meta{camel} {{"#)?;
    wl!(r#"    pub const XML_VERSION: u32 = {};"#, interface.version)?;
    wl!(r#"}}"#)?;
    Ok(())
}

fn format_interface_constructors(w: &mut impl Write, interface: &Interface) -> io::Result<()> {
    define_w!(w);
    let snake = &interface.name;
    let camel = format_camel(snake).to_string();
    wl!(r#"impl Meta{camel} {{"#)?;
    wl!(
        r#"    pub(crate) fn new(state: &Rc<InnerState>, version: u32) -> Rc<Self> {{"#
    )?;
    wl!(r#"        Rc::new(Self {{"#)?;
    wl!(r#"            core: ProxyCore::new(state, ProxyInterface::{camel}, version),"#)?;
    wl!(r#"            handler: Default::default(),"#)?;
    wl!(r#"        }})"#)?;
    wl!(r#"    }}"#)?;
    wl!(r#"}}"#)?;
    Ok(())
}

fn format_interface_message_functions(w: &mut impl Write, interface: &Interface) -> io::Result<()> {
    define_w!(w);
    if interface.messages.is_empty() {
        return Ok(());
    }
    let snake = &interface.name;
    let name = format_camel(snake).to_string();
    wl!(r#"impl Meta{name} {{"#)?;
    for (idx, message) in interface.messages.iter().enumerate() {
        if idx > 0 {
            wl!()?;
        }
        format_message_since(w, message)?;
        wl!()?;
        format_message_doc(w, true, message)?;
        wl!(r#"    #[inline]"#)?;
        w!(r#"    pub fn send_{}"#, &message.name)?;
        wl!(r#"("#)?;
        wl!(r#"        &self,"#)?;
        let num_args = message.args.len();
        for arg in &message.args {
            wl!(
                r#"        {}: {},"#,
                escape_name(&arg.name),
                arg_type(interface, arg),
            )?;
        }
        w!(r#"    ) -> Result<(), ObjectError>"#)?;
        wl!(r#" {{"#)?;
        if num_args > 0 {
            wl!(r#"        let ("#)?;
            for (idx, _) in message.args.iter().enumerate() {
                wl!(r#"            arg{idx},"#)?;
            }
            wl!(r#"        ) = ("#)?;
            for arg in &message.args {
                wl!(r#"            {},"#, escape_name(&arg.name))?;
            }
            wl!(r#"        );"#)?;
            for (idx, arg) in message.args.iter().enumerate() {
                if matches!(arg.ty, ArgType::NewId | ArgType::Object) {
                    let mut suffix = "";
                    if arg.ty == ArgType::NewId {
                        wl!(r#"        let arg{idx}_obj = arg{idx};"#)?;
                        suffix = "_obj";
                    }
                    if arg.allow_null {
                        wl!(r#"        let arg{idx} = arg{idx}{suffix}.map(|a| a.core());"#)?;
                    } else {
                        wl!(r#"        let arg{idx} = arg{idx}{suffix}.core();"#)?;
                    }
                }
            }
        }
        wl!(r#"        let core = self.core();"#)?;
        if message.is_request {
            wl!(r#"        let Some(id) = core.server_obj_id.get() else {{"#)?;
            wl!(r#"            return Err(ObjectError);"#)?;
            wl!(r#"        }};"#)?;
        } else {
            wl!(r#"        let client_ref = core.client.borrow();"#)?;
            wl!(r#"        let Some(client) = &*client_ref else {{"#)?;
            wl!(r#"            return Err(ObjectError);"#)?;
            wl!(r#"        }};"#)?;
        }
        if !message.is_request {
            for (idx, arg) in message.args.iter().enumerate() {
                if arg.ty == ArgType::Object {
                    let mut prefix = "";
                    if arg.allow_null {
                        prefix = "    ";
                        wl!(r#"        if let Some(arg{idx}) = arg{idx} {{"#)?;
                    }
                    wl!(
                        r#"        {prefix}if arg{idx}.client_id.get() != Some(client.endpoint.id) {{"#
                    )?;
                    wl!(r#"        {prefix}    return Err(ObjectError);"#)?;
                    wl!(r#"        {prefix}}}"#)?;
                    if arg.allow_null {
                        wl!(r#"        }}"#)?;
                    }
                }
            }
        }
        if message.is_request {
            for (idx, arg) in message.args.iter().enumerate() {
                if arg.ty == ArgType::Object {
                    let mut prefix = "";
                    w!(r#"        let arg{idx} = "#)?;
                    if arg.allow_null {
                        prefix = "    ";
                        wl!(r#"match arg{idx} {{"#)?;
                        wl!(r#"            None => 0,"#)?;
                        w!(r#"            Some(arg{idx}) => "#)?;
                    }
                    wl!(r#"match arg{idx}.server_obj_id.get() {{"#)?;
                    wl!(r#"            {prefix}None => return Err(ObjectError),"#)?;
                    wl!(r#"            {prefix}Some(id) => id,"#)?;
                    if arg.allow_null {
                        wl!(r#"            }},"#)?;
                    }
                    wl!(r#"        }};"#)?;
                }
            }
        }
        for (idx, arg) in message.args.iter().enumerate() {
            if arg.ty == ArgType::NewId {
                if message.is_request {
                    wl!(r#"        arg{idx}.generate_server_id(arg{idx}_obj.clone())?;"#)?;
                } else {
                    wl!(r#"        arg{idx}.generate_client_id(client, arg{idx}_obj.clone())?;"#)?;
                }
            }
        }
        if message.is_request {
            wl!(r#"        let endpoint = &self.core.state.server;"#)?;
        } else {
            wl!(r#"        let endpoint = &client.endpoint;"#)?;
        }
        wl!(r#"        if !endpoint.has_outgoing.replace(true) {{"#)?;
        wl!(
            r#"            self.core.state.flushable_endpoints.borrow_mut().push(endpoint.clone());"#
        )?;
        wl!(r#"        }}"#)?;
        wl!(r#"        let mut outgoing_ref = endpoint.outgoing.borrow_mut();"#)?;
        wl!(r#"        let outgoing = &mut *outgoing_ref;"#)?;
        wl!(r#"        let mut fmt = outgoing.formatter();"#)?;
        let mut words = vec![];
        if message.is_request {
            words.push("id".to_string());
        } else {
            words.push("core.client_obj_id.get().unwrap_or(0)".to_string());
        }
        words.push(format!("{}", message.message_id));
        macro_rules! flush_words {
            () => {{
                if words.len() > 0 {
                    wl!(r#"        fmt.words(["#)?;
                    for word in words.drain(..) {
                        wl!(r#"            {word},"#)?;
                    }
                    wl!(r#"        ]);"#)?;
                }
            }};
        }
        for (idx, arg) in message.args.iter().enumerate() {
            match arg.ty {
                ArgType::NewId | ArgType::Object => {
                    if message.is_request && arg.ty == ArgType::Object {
                        words.push(format!("arg{idx}"));
                    } else {
                        if arg.ty == ArgType::NewId && arg.interface.is_none() {
                            flush_words!();
                            wl!(r#"        fmt.string(arg{idx}.interface.name());"#)?;
                            words.push(format!("arg{idx}.version"));
                        }
                        let mut s = String::new();
                        if arg.allow_null {
                            write!(s, "arg{idx}.map(|arg{idx}| ").unwrap();
                        }
                        if message.is_request {
                            write!(s, "arg{idx}.server_obj_id.get()").unwrap();
                        } else {
                            write!(s, "arg{idx}.client_obj_id.get()").unwrap();
                        }
                        if arg.allow_null {
                            write!(s, ").flatten()").unwrap();
                        }
                        write!(s, ".unwrap_or(0)").unwrap();
                        words.push(s);
                    }
                }
                ArgType::Int if arg.enum_.is_some() => words.push(format!("arg{idx}.0 as u32")),
                ArgType::Int => words.push(format!("arg{idx} as u32")),
                ArgType::Uint if arg.enum_.is_some() => words.push(format!("arg{idx}.0")),
                ArgType::Uint => words.push(format!("arg{idx}")),
                ArgType::Fixed => words.push(format!("arg{idx}.to_wire() as u32")),
                ArgType::String => {
                    flush_words!();
                    if arg.allow_null {
                        wl!(r#"        if let Some(arg{idx}) = arg{idx} {{"#)?;
                        wl!(r#"            fmt.string(arg{idx});"#)?;
                        wl!(r#"        }}"#)?;
                    } else {
                        wl!(r#"        fmt.string(arg{idx});"#)?;
                    }
                }
                ArgType::Array => {
                    flush_words!();
                    wl!(r#"        fmt.array(arg{idx});"#)?;
                }
                ArgType::Fd => {
                    wl!(r#"        fmt.fds.push_back(arg{idx}.clone());"#)?;
                }
            }
        }
        flush_words!();
        if message.ty == Some(MessageType::Destructor) {
            if message.is_request {
                wl!(r#"        self.core.handle_server_destroy();"#)?;
            } else {
                wl!(r#"        drop(fmt);"#)?;
                wl!(r#"        drop(outgoing_ref);"#)?;
                wl!(r#"        drop(client_ref);"#)?;
                wl!(r#"        self.core.handle_client_destroy();"#)?;
            }
        }
        wl!(r#"        Ok(())"#)?;
        wl!(r#"    }}"#)?;
    }
    wl!(r#"}}"#)?;
    Ok(())
}

fn format_message_since(w: &mut impl Write, message: &Message) -> io::Result<()> {
    format_since(
        w,
        "MSG",
        "message",
        &message.name,
        format_uppercase(&message.name),
        message.since,
        message.deprecated_since,
    )
}

fn format_since(
    w: &mut impl Write,
    prefix: &str,
    ty: &str,
    name: impl Display,
    uppercase: impl Display,
    since: Option<u32>,
    deprecated_since: Option<u32>,
) -> io::Result<()> {
    define_w!(w);
    wl!(r#"    /// Since when the {name} {ty} is available."#,)?;
    wl!(r#"    #[allow(dead_code)]"#)?;
    wl!(
        r#"    pub const {prefix}__{uppercase}__SINCE: u32 = {};"#,
        since.unwrap_or(1),
    )?;
    if let Some(n) = deprecated_since {
        wl!()?;
        wl!(r#"    /// Since when the {name} {ty} is deprecated."#,)?;
        wl!(r#"    #[allow(dead_code)]"#)?;
        wl!(r#"    pub const {prefix}__{uppercase}__DEPRECATED_SINCE: u32 = {n};"#,)?;
    }
    Ok(())
}

fn format_interface_message_handler(
    w: &mut impl Write,
    mutable_data: bool,
    interface: &Interface,
) -> io::Result<()> {
    define_w!(w);
    let snake = &interface.name;
    let camel = format_camel(snake).to_string();
    wl!(r#"/// A message handler for [{camel}] proxies."#)?;
    wl!(r#"#[allow(dead_code)]"#)?;
    wl!(r#"pub trait Meta{camel}MessageHandler {{"#)?;
    if mutable_data {
        wl!(r#"    type Data: 'static;"#)?;
        wl!()?;
    }
    for (idx, msg) in interface.messages.iter().enumerate() {
        if idx > 0 {
            wl!()?;
        }
        format_message_doc(w, false, msg)?;
        wl!(r#"    #[inline]"#)?;
        wl!(r#"    fn {}("#, escape_name(&msg.name))?;
        wl!(r#"        &mut self,"#)?;
        if mutable_data {
            wl!(r#"        _data: &mut Self::Data,"#)?;
        }
        wl!(r#"        _slf: &Rc<Meta{camel}>,"#)?;
        for arg in &msg.args {
            wl!(
                r#"        {}: {},"#,
                escape_name(&arg.name),
                arg_type(interface, arg)
            )?;
        }
        wl!(r#"    ) {{"#)?;
        if !msg.is_request && msg.args.iter().any(|a| matches!(a.ty, ArgType::Object)) {
            wl!(r#"        if let Some(client_id) = _slf.core.client_id.get() {{"#)?;
            for arg in &msg.args {
                if arg.ty == ArgType::Object {
                    let mut prefix = "";
                    if arg.allow_null {
                        wl!(
                            r#"            if let Some({}) = {} {{"#,
                            escape_name(&arg.name),
                            escape_name(&arg.name),
                        )?;
                        prefix = "    ";
                    }
                    wl!(
                        r#"            {prefix}if let Some(client_id_2) = {}.core().client_id.get() {{"#,
                        escape_name(&arg.name)
                    )?;
                    wl!(r#"            {prefix}    if client_id != client_id_2 {{"#)?;
                    wl!(r#"            {prefix}        return;"#)?;
                    wl!(r#"            {prefix}    }}"#)?;
                    wl!(r#"            {prefix}}}"#)?;
                    if arg.allow_null {
                        wl!(r#"            }}"#)?;
                    }
                }
            }
            wl!(r#"        }}"#)?;
        }
        wl!(r#"        let res = _slf.send_{}("#, &msg.name)?;
        for arg in &msg.args {
            wl!(r#"            {},"#, escape_name(&arg.name))?;
        }
        wl!(r#"        );"#)?;
        wl!(r#"        if let Err(e) = res {{"#)?;
        wl!(
            r#"            log::warn!("Could not forward a {}.{} message: {{}}", Report::new(e));"#,
            interface.name,
            msg.name,
        )?;
        wl!(r#"        }}"#)?;
        wl!(r#"    }}"#)?;
    }
    wl!(r#"}}"#)?;
    Ok(())
}

fn arg_type<'a>(interface: &'a Interface, arg: &'a Arg) -> impl Display + use<'a> {
    debug_fn(move |f| {
        if let Some(enum_) = &arg.enum_ {
            if enum_.contains('.') {
                return write!(f, "Meta{}", format_camel(enum_));
            }
            return write!(
                f,
                "Meta{}{}",
                format_camel(&interface.name),
                format_camel(enum_)
            );
        }
        let s = match &arg.ty {
            ArgType::NewId => match &arg.interface {
                None => "Rc<dyn Proxy>",
                Some(s) => return write!(f, "&Rc<Meta{}>", format_camel(s)),
            },
            ArgType::Int => "i32",
            ArgType::Uint => "u32",
            ArgType::Fixed => "Fixed",
            ArgType::String if arg.allow_null => "Option<&str>",
            ArgType::String => "&str",
            ArgType::Object if arg.allow_null => match &arg.interface {
                None => "Option<Rc<dyn Proxy>>",
                Some(s) => {
                    return write!(f, "Option<&Rc<Meta{}>>", format_camel(s));
                }
            },
            ArgType::Object => match &arg.interface {
                None => "Rc<dyn Proxy>",
                Some(s) => {
                    return write!(f, "&Rc<Meta{}>", format_camel(s));
                }
            },
            ArgType::Array => "&[u8]",
            ArgType::Fd => "&Rc<OwnedFd>",
        };
        f.write_str(s)
    })
}

#[allow(clippy::type_complexity)]
pub fn format_mod_file(
    w: &mut impl Write,
    protocols: &[(String, Vec<(String, Vec<String>, u32)>)],
) -> io::Result<()> {
    define_w!(w);
    for (protocol, _) in protocols {
        wl!(r#"pub mod {};"#, protocol)?;
    }
    wl!()?;
    wl!("#[allow(unused_imports)]")?;
    wl!("mod all_types {{")?;
    for (proto, interfaces) in protocols {
        for (snake, enums, _) in interfaces {
            let camel = format_camel(snake).to_string();
            let prefix = debug_fn(|f| {
                write!(
                    f,
                    r#"    pub(super) use super::{proto}::{snake}::Meta{camel}"#
                )
            });
            wl!(r#"{prefix};"#)?;
            for enum_ in enums {
                wl!(r#"{prefix}{};"#, format_camel(enum_))?;
            }
        }
    }
    wl!()?;
    wl!("    use crate::generated_helper::prelude::*;")?;
    wl!()?;
    wl!(
        "    pub(super) fn create_proxy_for_interface(state: &Rc<InnerState>, interface: &str, version: u32) -> Result<Rc<dyn Proxy>, ObjectError> {{"
    )?;
    wl!(
        "        static INTERFACES: phf::Map<&'static str, fn(&Rc<InnerState>, u32) -> Result<Rc<dyn Proxy>, ObjectError>> = phf::phf_map! {{"
    )?;
    for (_, interfaces) in protocols {
        for (snake, _, _) in interfaces {
            let camel = format_camel(snake).to_string();
            wl!(r#"            "{snake}" => |s, v| {{"#)?;
            wl!(r#"                if v > Meta{camel}::XML_VERSION {{"#)?;
            wl!(r#"                    return Err(ObjectError);"#)?;
            wl!(r#"                }}"#)?;
            wl!(r#"                Ok(Meta{camel}::new(s, v))"#)?;
            wl!(r#"            }},"#)?;
        }
    }
    wl!("        }};")?;
    wl!("        INTERFACES.get(interface).ok_or(ObjectError).and_then(|i| i(state, version))")?;
    wl!("    }}")?;
    wl!()?;
    wl!("    pub(super) fn xml_interface_version(interface: &str) -> Option<u32> {{")?;
    wl!("        static INTERFACES: phf::Map<&'static str, u32> = phf::phf_map! {{")?;
    for (_, interfaces) in protocols {
        for (snake, _, version) in interfaces {
            wl!(r#"            "{snake}" => {version},"#)?;
        }
    }
    wl!("        }};")?;
    wl!("        INTERFACES.get(interface).copied()")?;
    wl!("    }}")?;
    wl!("}}")?;
    wl!()?;
    wl!("#[derive(Copy, Clone, Debug, Eq, PartialEq)]")?;
    wl!("pub enum ProxyInterface {{")?;
    for (_, interfaces) in protocols {
        for (snake, _, _) in interfaces {
            let camel = format_camel(snake);
            wl!(r#"    {camel},"#)?;
        }
    }
    wl!("}}")?;
    wl!()?;
    wl!("impl ProxyInterface {{")?;
    wl!("    pub fn name(self) -> &'static str {{")?;
    wl!("        match self {{")?;
    for (_, interfaces) in protocols {
        for (snake, _, _) in interfaces {
            let camel = format_camel(snake);
            wl!(r#"            Self::{camel} => "{snake}","#)?;
        }
    }
    wl!("        }}")?;
    wl!("    }}")?;
    wl!("}}")?;
    Ok(())
}

pub fn format_protocol_file(w: &mut impl Write, protocol: &Protocol) -> io::Result<()> {
    define_w!(w);
    if let Some(description) = &protocol.description {
        format_description(w, "//!", description)?;
        wl!()?;
    }
    wl!("#![allow(clippy::tabs_in_doc_comments)]")?;
    wl!("#![allow(clippy::doc_lazy_continuation)]")?;
    wl!("#![allow(clippy::too_many_arguments)]")?;
    wl!("#![allow(clippy::manual_map)]")?;
    wl!("#![allow(clippy::module_inception)]")?;
    wl!("#![allow(unused_imports)]")?;
    wl!("#![allow(rustdoc::broken_intra_doc_links)]")?;
    wl!("#![allow(rustdoc::bare_urls)]")?;
    wl!("#![allow(rustdoc::invalid_rust_codeblocks)]")?;
    wl!()?;
    for interface in &protocol.interfaces {
        let snake = &interface.name;
        wl!(r#"pub mod {snake};"#)?;
    }
    Ok(())
}

fn format_message_doc<W>(w: &mut W, request: bool, message: &Message) -> io::Result<()>
where
    W: Write,
{
    define_w!(w);
    let mut need_newline = false;
    if let Some(desc) = &message.description {
        format_description(w, "    ///", desc)?;
        need_newline = true;
    }
    let mut num_args = message.args.len();
    let mut has_object = false;
    if request {
        for arg in &message.args {
            if arg.ty == ArgType::NewId {
                num_args -= 1;
                break;
            }
        }
    } else {
        for arg in &message.args {
            if arg.ty == ArgType::Object {
                has_object = true;
                break;
            }
        }
    }
    if num_args > 0 {
        if need_newline {
            wl!("    ///")?;
        }
        wl!("    /// # Arguments")?;
        wl!("    ///")?;
        for arg in &message.args {
            let name = escape_name(&arg.name).to_string();
            w!("    /// - `{name}`:")?;
            let prefix = format!("    ///    {:width$}  ", " ", width = name.len());
            let mut first = true;
            let mut needs_newline = false;
            if let Some(summary) = &arg.summary {
                for line in summary.lines() {
                    if first {
                        first = false;
                    } else {
                        w!("{}", prefix)?;
                    }
                    wl!(" {}", line)?;
                }
                needs_newline = true;
            }
            if let Some(desc) = &arg.description {
                if needs_newline {
                    wl!("    ///")?;
                }
                format_description(w, &prefix, desc)?;
            }
            if arg.summary.is_none() && arg.description.is_none() {
                wl!()?;
            }
        }
        if has_object {
            wl!("    ///")?;
            wl!("    /// All borrowed proxies passed to this function are guaranteed to be")?;
            wl!("    /// immutable and non-null.")?;
        }
    }
    Ok(())
}

fn format_description(
    w: &mut impl Write,
    prefix: &str,
    description: &Description,
) -> io::Result<()> {
    define_w!(w);
    let mut needs_newline = false;
    if let Some(summary) = &description.summary {
        for line in summary.lines() {
            wl!("{prefix} {line}")?;
        }
        needs_newline = true;
    }
    let mut trim = None;
    let mut empty_lines = 0;
    'outer: for mut line in description.body.lines() {
        if trim.is_none() {
            let idx = 'idx: {
                for (idx, c) in line.char_indices() {
                    if c != ' ' && c != '\t' {
                        break 'idx idx;
                    }
                }
                continue 'outer;
            };
            trim = Some(&line[..idx]);
        }
        if let Some(stripped) = line.strip_prefix(trim.unwrap()) {
            line = stripped;
        }
        if line.trim_ascii().is_empty() {
            empty_lines += 1;
            continue;
        }
        if empty_lines > 0 {
            for _ in 0..empty_lines {
                wl!("{prefix}")?;
            }
            empty_lines = 0;
        }
        if needs_newline {
            needs_newline = false;
            wl!("{prefix}")?;
        }
        wl!("{prefix} {}", line)?;
    }
    Ok(())
}

fn format_proxy_impl(w: &mut impl Write, interface: &Interface) -> io::Result<()> {
    define_w!(w);
    let snake = &interface.name;
    let camel = format_camel(snake).to_string();
    wl!(r#"impl Proxy for Meta{camel} {{"#)?;
    wl!(r#"    fn core(&self) -> &ProxyCore {{"#)?;
    wl!(r#"        &self.core"#)?;
    wl!(r#"    }}"#)?;
    wl!()?;
    wl!(
        r#"    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {{"#
    )?;
    format_proxy_message_handler_body(w, interface, true)?;
    wl!(r#"    }}"#)?;
    wl!()?;
    wl!(
        r#"    fn handle_event(self: Rc<Self>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {{"#
    )?;
    format_proxy_message_handler_body(w, interface, false)?;
    wl!(r#"    }}"#)?;
    wl!(r#"}}"#)?;
    Ok(())
}

fn format_proxy_message_handler_body<W: Write>(
    w: &mut W,
    interface: &Interface,
    requests: bool,
) -> io::Result<()> {
    define_w!(w);
    let p = "        ";
    wl!(r#"{p}let handler = &mut *self.handler.borrow();"#)?;
    wl!(r#"{p}match msg[1] & 0xffff {{"#)?;
    let mut any_messages = false;
    for msg in &interface.messages {
        if msg.is_request != requests {
            continue;
        }
        any_messages = true;
        wl!(r#"{p}    {} => {{"#, msg.message_id)?;
        let num_words = msg.args.iter().try_fold(0, |n, a| {
            let size = match a.ty {
                ArgType::NewId if a.interface.is_none() => return None,
                ArgType::NewId
                | ArgType::Int
                | ArgType::Uint
                | ArgType::Fixed
                | ArgType::Object => 1,
                ArgType::Fd => 0,
                ArgType::String | ArgType::Array => return None,
            };
            Some(n + size)
        });
        if let Some(num_words) = num_words
            && num_words > 0
        {
            wl!(r#"{p}        let ["#)?;
            for (idx, arg) in msg.args.iter().enumerate() {
                match arg.ty {
                    ArgType::NewId
                    | ArgType::Int
                    | ArgType::Uint
                    | ArgType::Fixed
                    | ArgType::Object => {
                        wl!(r#"{p}            arg{idx},"#)?;
                    }
                    ArgType::Fd => continue,
                    ArgType::String | ArgType::Array => unreachable!(),
                }
            }
            wl!(r#"{p}        ] = msg[2..] else {{"#)?;
            wl!(r#"{p}            return Err(ObjectError);"#)?;
            wl!(r#"{p}        }};"#)?;
        } else if msg.args.iter().any(|a| !matches!(a.ty, ArgType::Fd)) {
            wl!(r#"{p}        let mut offset = 2;"#)?;
            let parse_array = |w: &mut W, ty: ArgType, allow_null: bool| -> io::Result<()> {
                define_w!(w, wl2);
                wl2!(r#"{p}            let Some(&len) = msg.get(offset) else {{"#)?;
                wl2!(r#"{p}                return Err(ObjectError);"#)?;
                wl2!(r#"{p}            }};"#)?;
                wl2!(r#"{p}            offset += 1;"#)?;
                wl2!(r#"{p}            let len = len as usize;"#)?;
                wl2!(r#"{p}            let words = ((len as u64 + 3) / 4) as usize;"#)?;
                wl2!(r#"{p}            if offset + words > msg.len() {{"#)?;
                wl2!(r#"{p}                return Err(ObjectError);"#)?;
                wl2!(r#"{p}            }}"#)?;
                wl2!(r#"{p}            let start = offset;"#)?;
                wl2!(r#"{p}            offset += words;"#)?;
                if ty == ArgType::Array {
                    wl2!(r#"{p}            &uapi::as_bytes(&msg[start..])[..len]"#)?;
                } else {
                    wl2!(r#"{p}            let bytes = &uapi::as_bytes(&msg[start..])[..len];"#)?;
                    wl2!(r#"{p}            if bytes.is_empty() {{"#)?;
                    if allow_null {
                        wl2!(r#"{p}                None"#)?;
                    } else {
                        wl2!(r#"{p}                return Err(ObjectError);"#)?;
                    }
                    wl2!(r#"{p}            }} else {{"#)?;
                    wl2!(
                        r#"{p}                let Ok(s) = str::from_utf8(&bytes[..len-1]) else {{"#
                    )?;
                    wl2!(r#"{p}                    return Err(ObjectError);"#)?;
                    wl2!(r#"{p}                }};"#)?;
                    if allow_null {
                        wl2!(r#"{p}                Some(s)"#)?;
                    } else {
                        wl2!(r#"{p}                s"#)?;
                    }
                    wl2!(r#"{p}            }}"#)?;
                }
                Ok(())
            };
            for (idx, arg) in msg.args.iter().enumerate() {
                match arg.ty {
                    ArgType::NewId
                    | ArgType::Int
                    | ArgType::Uint
                    | ArgType::Fixed
                    | ArgType::Object => {
                        if arg.ty == ArgType::NewId && arg.interface.is_none() {
                            wl!(r#"{p}        let arg{idx}_interface = {{"#)?;
                            parse_array(w, ArgType::String, false)?;
                            wl!(r#"{p}        }};"#)?;
                            wl!(
                                r#"{p}        let Some(&arg{idx}_version) = msg.get(offset) else {{"#
                            )?;
                            wl!(r#"{p}            return Err(ObjectError);"#)?;
                            wl!(r#"{p}        }};"#)?;
                            wl!(r#"{p}        offset += 1;"#)?;
                        }
                        wl!(r#"{p}        let Some(&arg{idx}) = msg.get(offset) else {{"#)?;
                        wl!(r#"{p}            return Err(ObjectError);"#)?;
                        wl!(r#"{p}        }};"#)?;
                        wl!(r#"{p}        offset += 1;"#)?;
                    }
                    ArgType::Fd => continue,
                    ArgType::String | ArgType::Array => {
                        wl!(r#"{p}        let arg{idx} = {{"#)?;
                        parse_array(w, arg.ty, arg.allow_null)?;
                        wl!(r#"{p}        }};"#)?;
                    }
                }
            }
            wl!(r#"{p}        if offset != msg.len() {{"#)?;
            wl!(r#"{p}            return Err(ObjectError);"#)?;
            wl!(r#"{p}        }}"#)?;
        }
        for (idx, arg) in msg.args.iter().enumerate() {
            if arg.ty == ArgType::Fd {
                wl!(r#"{p}        let Some(arg{idx}) = fds.pop_front() else {{"#)?;
                wl!(r#"{p}            return Err(ObjectError);"#)?;
                wl!(r#"{p}        }};"#)?;
            }
        }
        if interface.is_wl_registry && msg.name == "global" {
            wl!(r#"{p}        let arg2 = match xml_interface_version(arg1) {{"#)?;
            wl!(r#"{p}            Some(v) => v.min(arg2),"#)?;
            wl!(r#"{p}            _ => return Ok(()),"#)?;
            wl!(r#"{p}        }};"#)?;
        }
        for (idx, arg) in msg.args.iter().enumerate() {
            match arg.ty {
                ArgType::NewId => {
                    wl!(r#"{p}        let arg{idx}_id = arg{idx};"#)?;
                    match &arg.interface {
                        Some(interface) => {
                            let camel = format_camel(interface);
                            wl!(
                                r#"{p}        let arg{idx} = Meta{camel}::new(&self.core.state, self.core.version);"#
                            )?;
                        }
                        _ => {
                            wl!(
                                r#"{p}        let arg{idx} = create_proxy_for_interface(&self.core.state, arg{idx}_interface, arg{idx}_version)?;"#
                            )?;
                        }
                    }
                    if msg.is_request {
                        wl!(
                            r#"{p}        arg{idx}.core().set_client_id(client, arg{idx}_id, arg{idx}.clone())?;"#
                        )?;
                    } else {
                        wl!(
                            r#"{p}        arg{idx}.core().set_server_id(arg{idx}_id, arg{idx}.clone())?;"#
                        )?;
                    }
                }
                ArgType::Object => {
                    let mut prefix = "";
                    if arg.allow_null {
                        wl!(r#"{p}        let arg{idx} = if arg{idx} == 0 {{"#)?;
                        wl!(r#"{p}            None"#)?;
                        wl!(r#"{p}        }} else {{"#)?;
                        prefix = "    ";
                    }
                    if msg.is_request {
                        wl!(
                            r#"{p}        {prefix}let Some(arg{idx}) = client.endpoint.lookup(arg{idx}) else {{"#
                        )?;
                    } else {
                        wl!(
                            r#"{p}        {prefix}let Some(arg{idx}) = self.core.state.server.lookup(arg{idx}) else {{"#
                        )?;
                    }
                    wl!(r#"{p}            {prefix}return Err(ObjectError);"#)?;
                    wl!(r#"{p}        {prefix}}};"#)?;
                    if let Some(interface) = &arg.interface {
                        let camel = format_camel(interface);
                        wl!(
                            r#"{p}        {prefix}let Ok(arg{idx}) = (arg{idx} as Rc<dyn Any>).downcast::<Meta{camel}>() else {{"#
                        )?;
                        wl!(r#"{p}        {prefix}    return Err(ObjectError);"#)?;
                        wl!(r#"{p}        {prefix}}};"#)?;
                    }
                    if arg.allow_null {
                        wl!(r#"{p}            Some(arg{idx})"#)?;
                        wl!(r#"{p}        }};"#)?;
                    }
                }
                ArgType::Int => {}
                ArgType::Uint => {}
                ArgType::Fixed => {}
                ArgType::String => {}
                ArgType::Array => {}
                ArgType::Fd => {}
            }
        }
        for (idx, arg) in msg.args.iter().enumerate() {
            match arg.ty {
                ArgType::String | ArgType::Array => continue,
                ArgType::Uint if arg.enum_.is_none() => continue,
                ArgType::NewId | ArgType::Object if arg.interface.is_none() => continue,
                _ => {}
            }
            w!(r#"                let arg{idx} = "#)?;
            match arg.ty {
                ArgType::NewId | ArgType::Object => {
                    if arg.allow_null {
                        w!(r#"arg{idx}.as_ref()"#)?;
                    } else {
                        w!(r#"&arg{idx}"#)?;
                    }
                }
                ArgType::Int | ArgType::Uint if arg.enum_.is_some() => {
                    w!(r#"{}(arg{idx})"#, arg_type(interface, arg))?;
                }
                ArgType::Int => {
                    w!(r#"arg{idx} as i32"#)?;
                }
                ArgType::Uint | ArgType::String | ArgType::Array => {
                    unreachable!();
                }
                ArgType::Fixed => {
                    w!(r#"Fixed::from_wire(arg{idx} as i32)"#)?;
                }
                ArgType::Fd => {
                    w!(r#"&arg{idx}"#)?;
                }
            }
            wl!(r#";"#)?;
        }
        if interface.is_wl_display && !msg.is_request {
            if msg.name == "delete_id" {
                wl!(r#"{p}        self.core.state.handle_delete_id(arg0);"#)?;
            } else if msg.name == "error" {
                wl!(r#"{p}        self.core.state.handle_error(arg0, arg1, arg2);"#)?;
            }
        } else {
            macro_rules! format_call {
                ($target:expr) => {
                    w!(r#"{p}            {}.{}("#, $target, escape_name(&msg.name))?;
                    w!(r#"&self"#)?;
                    for (idx, _) in msg.args.iter().enumerate() {
                        w!(r#", arg{idx}"#)?;
                    }
                    wl!(r#");"#)?;
                };
            }
            wl!(r#"{p}        if let Some(handler) = handler {{"#)?;
            format_call!("(**handler)");
            wl!(r#"{p}        }} else {{"#)?;
            format_call!("DefaultMessageHandler");
            wl!(r#"{p}        }}"#)?;
            if msg.ty == Some(MessageType::Destructor) {
                if msg.is_request {
                    wl!(r#"{p}        self.core.handle_client_destroy();"#)?;
                } else {
                    wl!(r#"{p}        self.core.handle_server_destroy();"#)?;
                }
            }
        }
        wl!(r#"{p}    }}"#)?;
    }
    wl!(r#"{p}    _ => {{"#)?;
    if requests {
        wl!(r#"{p}        let _ = client;"#)?;
    }
    wl!(r#"{p}        let _ = msg;"#)?;
    wl!(r#"{p}        let _ = fds;"#)?;
    wl!(r#"{p}        let _ = handler;"#)?;
    wl!(r#"{p}        return Err(ObjectError);"#)?;
    wl!(r#"{p}    }}"#)?;
    wl!(r#"{p}}}"#)?;
    if any_messages {
        wl!(r#"{p}Ok(())"#)?;
    }
    Ok(())
}

fn format_interface_enums(w: &mut impl Write, interface: &Interface) -> io::Result<()> {
    define_w!(w);
    let camel = format_camel(&interface.name).to_string();
    if interface.enums.len() > 0 {
        wl!(r#"impl Meta{camel} {{"#)?;
        for (idx, enum_) in interface.enums.iter().enumerate() {
            if idx > 0 {
                wl!()?;
            }
            for entry in &enum_.entries {
                format_since(
                    w,
                    "ENM",
                    "enum variant",
                    debug_fn(|f| write!(f, "{}.{}", enum_.name, entry.name)),
                    debug_fn(|f| {
                        write!(
                            f,
                            "{}_{}",
                            format_uppercase(&enum_.name),
                            format_uppercase(&entry.name)
                        )
                    }),
                    entry.since,
                    entry.deprecated_since,
                )?;
            }
        }
        wl!(r#"}}"#)?;
        wl!()?;
    }
    for (idx, enum_) in interface.enums.iter().enumerate() {
        if idx > 0 {
            wl!()?;
        }
        let camel = format!("{camel}{}", format_camel(&enum_.name));
        if let Some(desc) = &enum_.description {
            format_description(w, "///", desc)?;
        }
        wl!(r#"#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]"#)?;
        if enum_.bitfield {
            wl!(r#"#[derive(Default)]"#)?;
        }
        wl!(r#"#[allow(dead_code)]"#)?;
        wl!(r#"pub struct Meta{camel}(pub u32);"#)?;
        if enum_.bitfield {
            wl!()?;
            wl!(r#"/// An iterator over the set bits in a [Meta{camel}]."#)?;
            wl!(r#"///"#)?;
            wl!(
                r#"/// You can construct this with the `IntoIterator` implementation of `Meta{camel}`."#
            )?;
            wl!(r#"#[derive(Clone, Debug)]"#)?;
            wl!(r#"pub struct Meta{camel}Iter(pub u32);"#)?;
        }
        if enum_.entries.len() > 0 {
            wl!()?;
            wl!(r#"impl Meta{camel} {{"#)?;
            for (idx, entry) in enum_.entries.iter().enumerate() {
                if idx > 0 {
                    wl!()?;
                }
                let mut needs_newline = false;
                if let Some(summary) = &entry.summary {
                    for line in summary.lines() {
                        wl!(r#"    /// {line}"#)?;
                        needs_newline = true;
                    }
                }
                if let Some(desc) = &entry.description {
                    if needs_newline {
                        wl!(r#"    ///"#)?;
                    }
                    format_description(w, "    ///", desc)?;
                }
                wl!(r#"    #[allow(dead_code)]"#)?;
                wl!(
                    r#"    pub const {}: Self = Self({});"#,
                    format_enum_variant(&entry.name),
                    entry.value
                )?;
            }
            wl!(r#"}}"#)?;
        }
        if enum_.bitfield {
            wl!()?;
            wl!(r#"#[allow(dead_code)]"#)?;
            wl!(r#"impl Meta{camel} {{"#)?;
            wl!(r#"    #[inline]"#)?;
            wl!(r#"    pub const fn empty() -> Self {{"#)?;
            wl!(r#"        Self(0)"#)?;
            wl!(r#"    }}"#)?;
            wl!()?;
            wl!(r#"    #[inline]"#)?;
            wl!(r#"    #[must_use]"#)?;
            wl!(r#"    pub const fn is_empty(self) -> bool {{"#)?;
            wl!(r#"        self.0 == 0"#)?;
            wl!(r#"    }}"#)?;
            wl!()?;
            wl!(r#"    #[inline]"#)?;
            wl!(r#"    #[must_use]"#)?;
            wl!(r#"    pub const fn contains(self, other: Self) -> bool {{"#)?;
            wl!(r#"        self.0 & other.0 == other.0"#)?;
            wl!(r#"    }}"#)?;
            wl!()?;
            wl!(r#"    #[inline]"#)?;
            wl!(r#"    #[must_use]"#)?;
            wl!(r#"    pub const fn intersects(self, other: Self) -> bool {{"#)?;
            wl!(r#"        self.0 & other.0 != 0"#)?;
            wl!(r#"    }}"#)?;
            wl!()?;
            wl!(r#"    #[inline]"#)?;
            wl!(r#"    pub const fn insert(&mut self, other: Self) {{"#)?;
            wl!(r#"        *self = self.union(other);"#)?;
            wl!(r#"    }}"#)?;
            wl!()?;
            wl!(r#"    #[inline]"#)?;
            wl!(r#"    pub const fn remove(&mut self, other: Self) {{"#)?;
            wl!(r#"        *self = self.difference(other);"#)?;
            wl!(r#"    }}"#)?;
            wl!()?;
            wl!(r#"    #[inline]"#)?;
            wl!(r#"    pub const fn toggle(&mut self, other: Self) {{"#)?;
            wl!(r#"        *self = self.symmetric_difference(other);"#)?;
            wl!(r#"    }}"#)?;
            wl!()?;
            wl!(r#"    #[inline]"#)?;
            wl!(r#"    pub const fn set(&mut self, other: Self, value: bool) {{"#)?;
            wl!(r#"        if value {{"#)?;
            wl!(r#"            self.insert(other);"#)?;
            wl!(r#"        }} else {{"#)?;
            wl!(r#"            self.remove(other);"#)?;
            wl!(r#"        }}"#)?;
            wl!(r#"    }}"#)?;
            wl!()?;
            wl!(r#"    #[inline]"#)?;
            wl!(r#"    #[must_use]"#)?;
            wl!(r#"    pub const fn intersection(self, other: Self) -> Self {{"#)?;
            wl!(r#"        Self(self.0 & other.0)"#)?;
            wl!(r#"    }}"#)?;
            wl!()?;
            wl!(r#"    #[inline]"#)?;
            wl!(r#"    #[must_use]"#)?;
            wl!(r#"    pub const fn union(self, other: Self) -> Self {{"#)?;
            wl!(r#"        Self(self.0 | other.0)"#)?;
            wl!(r#"    }}"#)?;
            wl!()?;
            wl!(r#"    #[inline]"#)?;
            wl!(r#"    #[must_use]"#)?;
            wl!(r#"    pub const fn difference(self, other: Self) -> Self {{"#)?;
            wl!(r#"        Self(self.0 & !other.0)"#)?;
            wl!(r#"    }}"#)?;
            wl!()?;
            wl!(r#"    #[inline]"#)?;
            wl!(r#"    #[must_use]"#)?;
            wl!(r#"    pub const fn complement(self) -> Self {{"#)?;
            wl!(r#"        Self(!self.0)"#)?;
            wl!(r#"    }}"#)?;
            wl!()?;
            wl!(r#"    #[inline]"#)?;
            wl!(r#"    #[must_use]"#)?;
            wl!(r#"    pub const fn symmetric_difference(self, other: Self) -> Self {{"#)?;
            wl!(r#"        Self(self.0 ^ other.0)"#)?;
            wl!(r#"    }}"#)?;
            wl!()?;
            wl!(r#"    #[inline]"#)?;
            wl!(r#"    pub const fn all_known() -> Self {{"#)?;
            wl!(r#"        #[allow(clippy::eq_op, clippy::identity_op)]"#)?;
            w!(r#"        Self(0"#)?;
            for entry in &enum_.entries {
                w!(r#" | {}"#, entry.value)?;
            }
            wl!(r#")"#)?;
            wl!(r#"    }}"#)?;
            wl!(r#"}}"#)?;
            wl!()?;
            wl!(r#"impl Iterator for Meta{camel}Iter {{"#)?;
            wl!(r#"    type Item = Meta{camel};"#)?;
            wl!()?;
            wl!(r#"    fn next(&mut self) -> Option<Self::Item> {{"#)?;
            wl!(r#"        if self.0 == 0 {{"#)?;
            wl!(r#"            return None;"#)?;
            wl!(r#"        }}"#)?;
            wl!(r#"        let bit = 1 << self.0.trailing_zeros();"#)?;
            wl!(r#"        self.0 &= !bit;"#)?;
            wl!(r#"        Some(Meta{camel}(bit))"#)?;
            wl!(r#"    }}"#)?;
            wl!(r#"}}"#)?;
            wl!()?;
            wl!(r#"impl IntoIterator for Meta{camel} {{"#)?;
            wl!(r#"    type Item = Meta{camel};"#)?;
            wl!(r#"    type IntoIter = Meta{camel}Iter;"#)?;
            wl!()?;
            wl!(r#"    fn into_iter(self) -> Self::IntoIter {{"#)?;
            wl!(r#"        Meta{camel}Iter(self.0)"#)?;
            wl!(r#"    }}"#)?;
            wl!(r#"}}"#)?;
            macro_rules! bitop {
                ($capital:literal, $lower:literal, $op:literal) => {{
                    wl!()?;
                    wl!(r#"impl Bit{} for Meta{camel} {{"#, $capital)?;
                    wl!(r#"    type Output = Self;"#)?;
                    wl!()?;
                    wl!(
                        r#"    fn bit{}(self, rhs: Self) -> Self::Output {{"#,
                        $lower
                    )?;
                    wl!(r#"        self.{}(rhs)"#, $op)?;
                    wl!(r#"    }}"#)?;
                    wl!(r#"}}"#)?;
                    wl!()?;
                    wl!(r#"impl Bit{}Assign for Meta{camel} {{"#, $capital)?;
                    wl!(r#"    fn bit{}_assign(&mut self, rhs: Self) {{"#, $lower)?;
                    wl!(r#"        *self = self.{}(rhs);"#, $op)?;
                    wl!(r#"    }}"#)?;
                    wl!(r#"}}"#)?;
                }};
            }
            bitop!("And", "and", "intersection");
            bitop!("Or", "or", "union");
            bitop!("Xor", "xor", "symmetric_difference");
            wl!()?;
            wl!(r#"impl Sub for Meta{camel} {{"#)?;
            wl!(r#"    type Output = Self;"#)?;
            wl!()?;
            wl!(r#"    fn sub(self, rhs: Self) -> Self::Output {{"#)?;
            wl!(r#"        self.difference(rhs)"#)?;
            wl!(r#"    }}"#)?;
            wl!(r#"}}"#)?;
            wl!()?;
            wl!(r#"impl SubAssign for Meta{camel} {{"#)?;
            wl!(r#"    fn sub_assign(&mut self, rhs: Self) {{"#)?;
            wl!(r#"        *self = self.difference(rhs);"#)?;
            wl!(r#"    }}"#)?;
            wl!(r#"}}"#)?;
            wl!()?;
            wl!(r#"impl Not for Meta{camel} {{"#)?;
            wl!(r#"    type Output = Self;"#)?;
            wl!()?;
            wl!(r#"    fn not(self) -> Self::Output {{"#)?;
            wl!(r#"        self.complement()"#)?;
            wl!(r#"    }}"#)?;
            wl!(r#"}}"#)?;
        }
        wl!()?;
        wl!(r#"impl Debug for Meta{camel} {{"#)?;
        wl!(r#"    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {{"#)?;
        if enum_.bitfield {
            wl!(r#"        let mut v = self.0;"#)?;
            wl!(r#"        let mut first = true;"#)?;
            let mut zero_entry = None;
            for entry in &enum_.entries {
                if entry.value_u32 == 0 {
                    zero_entry = Some(entry);
                    continue;
                }
                wl!(r#"        if v & {} == {} {{"#, entry.value, entry.value)?;
                wl!(r#"            v &= !{};"#, entry.value)?;
                wl!(r#"            if first {{"#)?;
                wl!(r#"                first = false;"#)?;
                wl!(r#"            }} else {{"#)?;
                wl!(r#"                f.write_str(" | ")?;"#)?;
                wl!(r#"            }}"#)?;
                wl!(
                    r#"            f.write_str("{}")?;"#,
                    format_enum_variant(&entry.name)
                )?;
                wl!(r#"        }}"#)?;
            }
            wl!(r#"        if v != 0 {{"#)?;
            wl!(r#"            if first {{"#)?;
            wl!(r#"                first = false;"#)?;
            wl!(r#"            }} else {{"#)?;
            wl!(r#"                f.write_str(" | ")?;"#)?;
            wl!(r#"            }}"#)?;
            wl!(r#"            write!(f, "0x{{v:032x}}")?;"#)?;
            wl!(r#"        }}"#)?;
            wl!(r#"        if first {{"#)?;
            if let Some(entry) = zero_entry {
                wl!(
                    r#"            f.write_str("{}")?;"#,
                    format_enum_variant(&entry.name)
                )?;
            } else {
                wl!(r#"            f.write_str("0")?;"#)?;
            }
            wl!(r#"        }}"#)?;
            wl!(r#"        Ok(())"#)?;
        } else {
            wl!(r#"        let name = match *self {{"#)?;
            for entry in &enum_.entries {
                let upper = format_enum_variant(&entry.name);
                wl!(r#"            Self::{upper} => "{upper}","#)?;
            }
            wl!(r#"            _ => return Debug::fmt(&self.0, f),"#)?;
            wl!(r#"        }};"#)?;
            wl!(r#"        f.write_str(name)"#)?;
        }
        wl!(r#"    }}"#)?;
        wl!(r#"}}"#)?;
    }
    Ok(())
}

fn format_interface_trait_impls(w: &mut impl Write, interface: &Interface) -> io::Result<()> {
    define_w!(w);
    let snake = &interface.name;
    let camel = format_camel(snake).to_string();
    wl!(r#"impl Debug for Meta{camel} {{"#)?;
    wl!(r#"    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {{"#)?;
    wl!(r#"        f.debug_struct("Meta{camel}")"#)?;
    wl!(r#"            .field("server_obj_id", &self.core.server_obj_id.get())"#)?;
    wl!(r#"            .field("client_id", &self.core.client_id.get())"#)?;
    wl!(r#"            .field("client_obj_id", &self.core.client_obj_id.get())"#)?;
    wl!(r#"            .finish()"#)?;
    wl!(r#"    }}"#)?;
    wl!(r#"}}"#)?;
    Ok(())
}

fn escape_name(name: &str) -> impl Display + use<'_> {
    static KEYWORDS: phf::Set<&'static str> = phf_set! {
        "abstract", "as", "async", "await", "become", "box", "break", "const",
        "continue", "crate", "do", "dyn", "else", "enum", "extern", "false", "final",
        "fn", "for", "gen", "if", "impl", "in", "let", "loop", "macro", "macro_rules",
        "match", "mod", "move", "mut", "override", "priv", "pub", "raw", "ref",
        "return", "safe", "self", "Self", "static", "struct", "super", "trait", "true",
        "try", "type", "typeof", "union", "unsafe", "unsized", "use", "virtual",
        "where", "while", "yield",
    };
    debug_fn(move |f| {
        if KEYWORDS.contains(name) {
            f.write_str("r#")?;
        }
        f.write_str(name)
    })
}
