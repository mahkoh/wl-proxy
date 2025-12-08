use {
    crate::{
        ast::{Arg, ArgType, Description, Interface, Message, MessageType, Protocol},
        collector::Suite,
    },
    debug_fn::debug_fn,
    isnt::std_1::primitive::IsntSliceExt,
    phf::phf_set,
    std::{
        collections::HashSet,
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
    wl!("use crate::protocol_helpers::prelude::*;")?;
    wl!("use super::super::all_types::*;")?;
    Ok(())
}

pub fn format_interface_file(w: &mut impl Write, interface: &Interface) -> io::Result<()> {
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
    format_interface_message_handler(w, interface)?;
    wl!()?;
    format_object_impl(w, interface)?;
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

const PREFIX: &str = "";

fn format_interface_types(w: &mut impl Write, interface: &Interface) -> io::Result<()> {
    define_w!(w);
    let snake = &interface.name;
    let camel = format_camel(snake).to_string();
    wl!(r#"/// A {snake} object."#)?;
    wl!(r#"///"#)?;
    wl!(r#"/// See the documentation of [the module][self] for the interface description."#)?;
    wl!(r#"pub struct {PREFIX}{camel} {{"#)?;
    wl!(r#"    core: ObjectCore,"#)?;
    wl!(r#"    handler: HandlerHolder<dyn {PREFIX}{camel}Handler>,"#)?;
    if interface.is_wl_registry {
        wl!(r#"    names: RefCell<HashSet<u32>>,"#)?;
    }
    wl!(r#"}}"#)?;
    if interface.messages.is_not_empty() {
        wl!()?;
        wl!(r#"struct DefaultHandler;"#)?;
        wl!()?;
        wl!(r#"impl {PREFIX}{camel}Handler for DefaultHandler {{ }}"#)?;
    }
    wl!()?;
    wl!(r#"impl ConcreteObject for {PREFIX}{camel} {{"#)?;
    wl!(r#"    const XML_VERSION: u32 = {};"#, interface.version)?;
    wl!(
        r#"    const INTERFACE: ObjectInterface = ObjectInterface::{};"#,
        camel
    )?;
    wl!(r#"    const INTERFACE_NAME: &str = "{}";"#, interface.name)?;
    wl!(r#"}}"#)?;
    Ok(())
}

fn format_interface_constructors(w: &mut impl Write, interface: &Interface) -> io::Result<()> {
    define_w!(w);
    let snake = &interface.name;
    let camel = format_camel(snake).to_string();
    wl!(r#"impl {PREFIX}{camel} {{"#)?;
    wl!(r#"    /// Sets a new handler."#)?;
    wl!(r#"    pub fn set_handler(&self, handler: impl {PREFIX}{camel}Handler) {{"#)?;
    wl!(r#"        self.set_boxed_handler(Box::new(handler));"#)?;
    wl!(r#"    }}"#)?;
    wl!()?;
    wl!(r#"    /// Sets a new, already boxed handler."#)?;
    wl!(r#"    pub fn set_boxed_handler(&self, handler: Box<dyn {PREFIX}{camel}Handler>) {{"#)?;
    wl!(r#"        if self.core.state.destroyed.get() {{"#)?;
    wl!(r#"            return;"#)?;
    wl!(r#"        }}"#)?;
    wl!(r#"        self.handler.set(Some(handler));"#)?;
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
    wl!(r#"impl {PREFIX}{name} {{"#)?;
    for (idx, msg) in interface.messages.iter().enumerate() {
        if idx > 0 {
            wl!()?;
        }
        format_message_since(w, msg)?;
        wl!()?;
        format_message_doc(w, true, false, msg)?;
        wl!(r#"    #[inline]"#)?;
        w!(r#"    pub fn try_send_{}"#, &msg.name)?;
        wl!(r#"("#)?;
        wl!(r#"        &self,"#)?;
        let num_args = msg.args.len();
        for (idx, arg) in msg.args.iter().enumerate() {
            let name = escape_name(&arg.name);
            if interface.is_wl_registry && msg.name == "global" && idx == 1 {
                wl!(r#"        {name}: ObjectInterface,"#)?;
            } else {
                wl!(r#"        {name}: {},"#, arg_type(interface, arg))?;
            }
        }
        w!(r#"    ) -> Result<(), ObjectError>"#)?;
        wl!(r#" {{"#)?;
        if interface.is_wl_registry && msg.name == "global" {
            wl!(r#"        let {0} = {0}.name();"#, msg.args[1].name)?;
        }
        if num_args > 0 {
            wl!(r#"        let ("#)?;
            for (idx, _) in msg.args.iter().enumerate() {
                wl!(r#"            arg{idx},"#)?;
            }
            wl!(r#"        ) = ("#)?;
            for arg in &msg.args {
                wl!(r#"            {},"#, escape_name(&arg.name))?;
            }
            wl!(r#"        );"#)?;
            for (idx, arg) in msg.args.iter().enumerate() {
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
        if msg.is_request {
            wl!(r#"        let Some(id) = core.server_obj_id.get() else {{"#)?;
            wl!(r#"            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));"#)?;
            wl!(r#"        }};"#)?;
        } else {
            wl!(r#"        let client_ref = core.client.borrow();"#)?;
            wl!(r#"        let Some(client) = &*client_ref else {{"#)?;
            wl!(r#"            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));"#)?;
            wl!(r#"        }};"#)?;
            wl!(r#"        let id = core.client_obj_id.get().unwrap_or(0);"#)?;
        }
        if !msg.is_request {
            for (idx, arg) in msg.args.iter().enumerate() {
                if arg.ty == ArgType::Object {
                    let mut prefix = "";
                    if arg.allow_null {
                        prefix = "    ";
                        wl!(r#"        if let Some(arg{idx}) = arg{idx} {{"#)?;
                    }
                    wl!(
                        r#"        {prefix}if arg{idx}.client_id.get() != Some(client.endpoint.id) {{"#
                    )?;
                    wl!(
                        r#"        {prefix}    return Err(ObjectError(ObjectErrorKind::ArgNoClientId("{}", client.endpoint.id)));"#,
                        arg.name
                    )?;
                    wl!(r#"        {prefix}}}"#)?;
                    if arg.allow_null {
                        wl!(r#"        }}"#)?;
                    }
                }
            }
        }
        if msg.is_request {
            for (idx, arg) in msg.args.iter().enumerate() {
                if arg.ty == ArgType::Object {
                    let mut prefix = "";
                    w!(r#"        let arg{idx}_id = "#)?;
                    if arg.allow_null {
                        prefix = "    ";
                        wl!(r#"match arg{idx} {{"#)?;
                        wl!(r#"            None => 0,"#)?;
                        w!(r#"            Some(arg{idx}) => "#)?;
                    }
                    wl!(r#"match arg{idx}.server_obj_id.get() {{"#)?;
                    wl!(
                        r#"            {prefix}None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("{}"))),"#,
                        arg.name
                    )?;
                    wl!(r#"            {prefix}Some(id) => id,"#)?;
                    if arg.allow_null {
                        wl!(r#"            }},"#)?;
                    }
                    wl!(r#"        }};"#)?;
                }
            }
        }
        for (idx, arg) in msg.args.iter().enumerate() {
            if arg.ty == ArgType::NewId {
                if msg.is_request {
                    wl!(r#"        arg{idx}.generate_server_id(arg{idx}_obj.clone())"#)?;
                    wl!(
                        r#"            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("{}", e)))?;"#,
                        arg.name
                    )?;
                } else {
                    wl!(r#"        arg{idx}.generate_client_id(client, arg{idx}_obj.clone())"#)?;
                    wl!(
                        r#"            .map_err(|e| ObjectError(ObjectErrorKind::GenerateClientId("{}", e)))?;"#,
                        arg.name
                    )?;
                }
            }
        }
        for (idx, arg) in msg.args.iter().enumerate() {
            if (arg.ty == ArgType::Object && !msg.is_request) || arg.ty == ArgType::NewId {
                w!(r#"        let arg{idx}_id = "#)?;
                if arg.allow_null {
                    w!("arg{idx}.and_then(|arg{idx}| ").unwrap();
                }
                if msg.is_request {
                    w!("arg{idx}.server_obj_id.get()").unwrap();
                } else {
                    w!("arg{idx}.client_obj_id.get()").unwrap();
                }
                if arg.allow_null {
                    w!(")").unwrap();
                }
                wl!(".unwrap_or(0);").unwrap();
            }
        }
        format_wayland_debug(w, interface, msg, true)?;
        if msg.is_request {
            wl!(r#"        let Some(endpoint) = &self.core.state.server else {{"#)?;
            wl!(r#"            return Ok(());"#)?;
            wl!(r#"        }};"#)?;
        } else {
            wl!(r#"        let endpoint = &client.endpoint;"#)?;
        }
        wl!(r#"        if !endpoint.flush_queued.replace(true) {{"#)?;
        if msg.is_request {
            wl!(r#"            self.core.state.add_flushable_endpoint(endpoint, None);"#)?;
        } else {
            wl!(r#"            self.core.state.add_flushable_endpoint(endpoint, Some(client));"#)?;
        }
        wl!(r#"        }}"#)?;
        wl!(r#"        let mut outgoing_ref = endpoint.outgoing.borrow_mut();"#)?;
        wl!(r#"        let outgoing = &mut *outgoing_ref;"#)?;
        wl!(r#"        let mut fmt = outgoing.formatter();"#)?;
        let mut words = vec![];
        words.push("id".to_string());
        words.push(format!("{}", msg.message_id));
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
        for (idx, arg) in msg.args.iter().enumerate() {
            match arg.ty {
                ArgType::NewId => {
                    if arg.interface.is_none() {
                        flush_words!();
                        wl!(r#"        fmt.string(arg{idx}.interface.name());"#)?;
                        words.push(format!("arg{idx}.version"));
                    }
                    words.push(format!("arg{idx}_id"));
                }
                ArgType::Object => words.push(format!("arg{idx}_id")),
                ArgType::Int | ArgType::Uint if arg.enum_.is_some() => {
                    words.push(format!("arg{idx}.0"))
                }
                ArgType::Int => words.push(format!("arg{idx} as u32")),
                ArgType::Uint => words.push(format!("arg{idx}")),
                ArgType::Fixed => words.push(format!("arg{idx}.to_wire() as u32")),
                ArgType::String => {
                    flush_words!();
                    if arg.allow_null {
                        wl!(r#"        if let Some(arg{idx}) = arg{idx} {{"#)?;
                        wl!(r#"            fmt.string(arg{idx});"#)?;
                        wl!(r#"        }} else {{"#)?;
                        wl!(r#"            fmt.words([0]);"#)?;
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
        if msg.ty == Some(MessageType::Destructor) {
            if msg.is_request {
                wl!(r#"        self.core.handle_server_destroy();"#)?;
            } else {
                wl!(r#"        drop(fmt);"#)?;
                wl!(r#"        drop(outgoing_ref);"#)?;
                wl!(r#"        drop(client_ref);"#)?;
                wl!(r#"        self.core.handle_client_destroy();"#)?;
            }
        }
        if interface.is_wl_fixes && msg.name == "destroy_registry" {
            wl!(r#"        arg0.handle_server_destroy();"#)?;
        }
        wl!(r#"        Ok(())"#)?;
        wl!(r#"    }}"#)?;
        wl!()?;
        format_message_doc(w, true, false, msg)?;
        wl!(r#"    #[inline]"#)?;
        w!(r#"    pub fn send_{}"#, &msg.name)?;
        wl!(r#"("#)?;
        wl!(r#"        &self,"#)?;
        for (idx, arg) in msg.args.iter().enumerate() {
            let name = escape_name(&arg.name);
            if interface.is_wl_registry && msg.name == "global" && idx == 1 {
                wl!(r#"        {name}: ObjectInterface,"#)?;
            } else {
                wl!(r#"        {name}: {},"#, arg_type(interface, arg))?;
            }
        }
        wl!(r#"    ) {{"#)?;
        wl!(r#"        let res = self.try_send_{}("#, msg.name)?;
        for arg in &msg.args {
            let name = escape_name(&arg.name);
            wl!(r#"            {name},"#)?;
        }
        wl!(r#"        );"#)?;
        wl!(r#"        if let Err(e) = res {{"#)?;
        wl!(
            r#"            log_send("{}.{}", &e);"#,
            interface.name,
            msg.name,
        )?;
        wl!(r#"        }}"#)?;
        wl!(r#"    }}"#)?;
        let new_id = msg
            .args
            .iter()
            .find(|a| a.ty == ArgType::NewId && a.interface.is_some());
        if let Some(new_id) = new_id {
            let new_interface_snake = new_id.interface.as_ref().unwrap();
            let new_interface_camel = format_camel(new_interface_snake);
            wl!()?;
            format_message_doc(w, true, true, msg)?;
            wl!(r#"    #[inline]"#)?;
            w!(r#"    pub fn new_try_send_{}"#, &msg.name)?;
            wl!(r#"("#)?;
            wl!(r#"        &self,"#)?;
            for (idx, arg) in msg.args.iter().enumerate() {
                if arg.ty == ArgType::NewId {
                    continue;
                }
                let name = escape_name(&arg.name);
                if interface.is_wl_registry && msg.name == "global" && idx == 1 {
                    wl!(r#"        {name}: ObjectInterface,"#)?;
                } else {
                    wl!(r#"        {name}: {},"#, arg_type(interface, arg))?;
                }
            }
            wl!(r#"    ) -> Result<Rc<{new_interface_camel}>, ObjectError> {{"#)?;
            wl!(
                r#"        let {} = self.core.create_child();"#,
                escape_name(&new_id.name)
            )?;
            wl!(r#"        self.try_send_{}("#, msg.name)?;
            for arg in &msg.args {
                let mut ref_ = "";
                if arg.ty == ArgType::NewId {
                    ref_ = "&";
                }
                let name = escape_name(&arg.name);
                wl!(r#"            {ref_}{name},"#)?;
            }
            wl!(r#"        )?;"#)?;
            wl!(r#"        Ok({})"#, escape_name(&new_id.name))?;
            wl!(r#"    }}"#)?;
            wl!()?;
            format_message_doc(w, true, true, msg)?;
            wl!(r#"    #[inline]"#)?;
            w!(r#"    pub fn new_send_{}"#, &msg.name)?;
            wl!(r#"("#)?;
            wl!(r#"        &self,"#)?;
            for (idx, arg) in msg.args.iter().enumerate() {
                if arg.ty == ArgType::NewId {
                    continue;
                }
                let name = escape_name(&arg.name);
                if interface.is_wl_registry && msg.name == "global" && idx == 1 {
                    wl!(r#"        {name}: ObjectInterface,"#)?;
                } else {
                    wl!(r#"        {name}: {},"#, arg_type(interface, arg))?;
                }
            }
            wl!(r#"    ) -> Rc<{new_interface_camel}> {{"#)?;
            wl!(
                r#"        let {} = self.core.create_child();"#,
                escape_name(&new_id.name)
            )?;
            wl!(r#"        self.send_{}("#, msg.name)?;
            for arg in &msg.args {
                let mut ref_ = "";
                if arg.ty == ArgType::NewId {
                    ref_ = "&";
                }
                let name = escape_name(&arg.name);
                wl!(r#"            {ref_}{name},"#)?;
            }
            wl!(r#"        );"#)?;
            wl!(r#"        {}"#, escape_name(&new_id.name))?;
            wl!(r#"    }}"#)?;
        }
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
    wl!(
        r#"    pub const {prefix}__{uppercase}__SINCE: u32 = {};"#,
        since.unwrap_or(1),
    )?;
    if let Some(n) = deprecated_since {
        wl!()?;
        wl!(r#"    /// Since when the {name} {ty} is deprecated."#,)?;
        wl!(r#"    pub const {prefix}__{uppercase}__DEPRECATED_SINCE: u32 = {n};"#,)?;
    }
    Ok(())
}

fn format_interface_message_handler(w: &mut impl Write, interface: &Interface) -> io::Result<()> {
    define_w!(w);
    let snake = &interface.name;
    let camel = format_camel(snake).to_string();
    wl!(r#"/// A message handler for [`{camel}`] proxies."#)?;
    wl!(r#"pub trait {PREFIX}{camel}Handler: Any {{"#)?;
    wl!(
        r#"    /// Event handler for wl_display.delete_id messages deleting the ID of this object."#
    )?;
    wl!(r#"    ///"#)?;
    wl!(r#"    /// The default handler forwards the event to the client, if any."#)?;
    wl!(r#"    #[inline]"#)?;
    wl!(r#"    fn delete_id(&mut self, slf: &Rc<{camel}>) {{"#)?;
    wl!(r#"        slf.core.delete_id();"#)?;
    wl!(r#"    }}"#)?;
    for msg in &interface.messages {
        if interface.is_wl_display && matches!(&*msg.name, "delete_id" | "error") {
            continue;
        }
        let mut arg_names = HashSet::new();
        for arg in &msg.args {
            arg_names.insert(escape_name(&arg.name).to_string());
        }
        let mut slf = "slf".to_string();
        while arg_names.contains(&slf) {
            slf.push_str("_");
        }
        wl!()?;
        format_message_doc(w, false, false, msg)?;
        wl!(r#"    #[inline]"#)?;
        wl!(r#"    fn handle_{}("#, msg.name)?;
        wl!(r#"        &mut self,"#)?;
        wl!(r#"        {slf}: &Rc<{PREFIX}{camel}>,"#)?;
        for (idx, arg) in msg.args.iter().enumerate() {
            let name = escape_name(&arg.name);
            if interface.is_wl_registry && msg.name == "global" && idx == 1 {
                wl!(r#"        {name}: ObjectInterface,"#)?;
            } else {
                wl!(r#"        {name}: {},"#, arg_type(interface, arg))?;
            }
        }
        wl!(r#"    ) {{"#)?;
        if msg.is_request {
            wl!(r#"        if !{slf}.core.forward_to_server.get() {{"#)?;
            wl!(r#"            return;"#)?;
            wl!(r#"        }}"#)?;
        } else {
            wl!(r#"        if !{slf}.core.forward_to_client.get() {{"#)?;
            wl!(r#"            return;"#)?;
            wl!(r#"        }}"#)?;
        }
        if !msg.is_request && msg.args.iter().any(|a| matches!(a.ty, ArgType::Object)) {
            wl!(r#"        if let Some(client_id) = {slf}.core.client_id.get() {{"#)?;
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
        wl!(r#"        let res = {slf}.try_send_{}("#, &msg.name)?;
        for arg in &msg.args {
            wl!(r#"            {},"#, escape_name(&arg.name))?;
        }
        wl!(r#"        );"#)?;
        wl!(r#"        if let Err(e) = res {{"#)?;
        wl!(
            r#"            log_forward("{}.{}", &e);"#,
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
                return write!(f, "{PREFIX}{}", format_camel(enum_));
            }
            return write!(
                f,
                "{PREFIX}{}{}",
                format_camel(&interface.name),
                format_camel(enum_)
            );
        }
        let s = match &arg.ty {
            ArgType::NewId => match &arg.interface {
                None => "Rc<dyn Object>",
                Some(s) => return write!(f, "&Rc<{PREFIX}{}>", format_camel(s)),
            },
            ArgType::Int => "i32",
            ArgType::Uint => "u32",
            ArgType::Fixed => "Fixed",
            ArgType::String if arg.allow_null => "Option<&str>",
            ArgType::String => "&str",
            ArgType::Object if arg.allow_null => match &arg.interface {
                None => "Option<Rc<dyn Object>>",
                Some(s) => {
                    return write!(f, "Option<&Rc<{PREFIX}{}>>", format_camel(s));
                }
            },
            ArgType::Object => match &arg.interface {
                None => "Rc<dyn Object>",
                Some(s) => {
                    return write!(f, "&Rc<{PREFIX}{}>", format_camel(s));
                }
            },
            ArgType::Array => "&[u8]",
            ArgType::Fd => "&Rc<OwnedFd>",
        };
        f.write_str(s)
    })
}

pub fn format_mod_file(w: &mut impl Write, suits: &[Suite]) -> io::Result<()> {
    define_w!(w);
    macro_rules! write_cfg {
        ($protocol:expr, $prefix:expr) => {
            if $protocol.is_wlproxy_test {
                wl!(r#"{}#[cfg(test)]"#, $prefix)?;
            } else if !$protocol.is_wayland {
                wl!(
                    r#"{}#[cfg(feature = "protocol-{}")]"#,
                    $prefix,
                    $protocol.name
                )?;
            }
        };
    }
    let protocols = || suits.iter().flat_map(|s| s.protocols.iter());
    let interfaces = || protocols().flat_map(|p| p.interfaces.iter().map(move |i| (p, i)));
    for protocol in protocols() {
        write_cfg!(protocol, "");
        wl!(r#"pub mod {};"#, protocol.name)?;
    }
    wl!()?;
    wl!("#[allow(unused_imports)]")?;
    wl!("mod all_types {{")?;
    for (protocol, interface) in interfaces() {
        let proto = &protocol.name;
        let snake = &interface.name;
        let camel = format_camel(snake).to_string();
        let prefix = debug_fn(|f| {
            write!(
                f,
                r#"    pub(super) use super::{proto}::{snake}::{PREFIX}{camel}"#
            )
        });
        write_cfg!(protocol, "    ");
        wl!(r#"{prefix};"#)?;
        for enum_ in &interface.enums {
            write_cfg!(protocol, "    ");
            wl!(r#"{prefix}{};"#, format_camel(&enum_.name))?;
        }
    }
    wl!()?;
    wl!("    use crate::protocol_helpers::prelude::*;")?;
    wl!()?;
    wl!(
        "    pub(super) fn create_object_for_interface(state: &Rc<State>, interface: &str, version: u32) -> Result<Rc<dyn Object>, ObjectError> {{"
    )?;
    wl!("        ObjectInterface::from_str(interface)")?;
    wl!(
        "            .ok_or(ObjectError(ObjectErrorKind::UnsupportedInterface(interface.to_string())))"
    )?;
    wl!("            .and_then(|i| i.create_object(state, version))")?;
    wl!("    }}")?;
    wl!()?;
    wl!("    impl ObjectInterface {{")?;
    wl!("        #[expect(clippy::should_implement_trait)]")?;
    wl!("        pub fn from_str(interface: &str) -> Option<ObjectInterface> {{")?;
    wl!(
        "            static INTERFACES: phf::Map<&'static str, Option<ObjectInterface>> = phf::phf_map! {{"
    )?;
    for (protocol, interface) in interfaces() {
        let snake = &interface.name;
        let camel = format_camel(snake);
        if protocol.is_wayland {
            wl!(r#"                "{snake}" => Some(ObjectInterface::{camel}),"#)?;
        } else if protocol.is_wlproxy_test {
            wl!(r#"                "{snake}" => {{"#)?;
            wl!(r#"                    #[cfg(test)] {{ Some(ObjectInterface::{camel}) }}"#)?;
            wl!(r#"                    #[cfg(not(test))] {{ None }}"#)?;
            wl!(r#"                }},"#)?;
        } else {
            let proto = &protocol.name;
            wl!(r#"                "{snake}" => {{"#)?;
            wl!(
                r#"                    #[cfg(feature = "protocol-{proto}")] {{ Some(ObjectInterface::{camel}) }}"#
            )?;
            wl!(r#"                    #[cfg(not(feature = "protocol-{proto}"))] {{ None }}"#)?;
            wl!(r#"                }},"#)?;
        }
    }
    wl!("            }};")?;
    wl!("            INTERFACES.get(interface).copied().flatten()")?;
    wl!("        }}")?;
    wl!()?;
    wl!(
        "        fn create_object(self, state: &Rc<State>, version: u32) -> Result<Rc<dyn Object>, ObjectError> {{"
    )?;
    wl!("            match self {{")?;
    for (protocol, interface) in interfaces() {
        let camel = format_camel(&interface.name);
        write_cfg!(protocol, "                ");
        wl!(r#"                Self::{camel} => {{"#)?;
        wl!(r#"                    if version > {PREFIX}{camel}::XML_VERSION {{"#)?;
        wl!(
            r#"                        return Err(ObjectError(ObjectErrorKind::MaxVersion(self, version)));"#
        )?;
        wl!(r#"                    }}"#)?;
        wl!(r#"                    Ok({PREFIX}{camel}::new(state, version))"#)?;
        wl!(r#"                }}"#)?;
    }
    wl!("            }}")?;
    wl!("        }}")?;
    wl!("    }}")?;
    wl!("}}")?;
    wl!()?;
    wl!("#[derive(Copy, Clone, Debug, Eq, PartialEq, linearize::Linearize)]")?;
    wl!("#[linearize(const)]")?;
    wl!("pub enum ObjectInterface {{")?;
    for (protocol, interface) in interfaces() {
        let snake = &interface.name;
        let camel = format_camel(snake);
        wl!(r#"    /// {snake}"#)?;
        write_cfg!(protocol, "    ");
        wl!(r#"    {camel},"#)?;
    }
    wl!("}}")?;
    wl!()?;
    wl!("impl ObjectInterface {{")?;
    wl!("    pub const fn name(self) -> &'static str {{")?;
    wl!("        match self {{")?;
    for (protocol, interface) in interfaces() {
        let snake = &interface.name;
        let camel = format_camel(snake);
        write_cfg!(protocol, "            ");
        wl!(r#"            Self::{camel} => "{snake}","#)?;
    }
    wl!("        }}")?;
    wl!("    }}")?;
    wl!()?;
    wl!("    pub const fn xml_version(self) -> u32 {{")?;
    wl!("        match self {{")?;
    for (protocol, interface) in interfaces() {
        let version = interface.version;
        let camel = format_camel(&interface.name);
        write_cfg!(protocol, "            ");
        wl!(r#"            Self::{camel} => {version},"#)?;
    }
    wl!("        }}")?;
    wl!("    }}")?;
    wl!("}}")?;
    Ok(())
}

pub fn format_baseline_file(w: &mut impl Write, suits: &[Suite]) -> io::Result<()> {
    define_w!(w);
    let protocols = || suits.iter().flat_map(|s| s.protocols.iter());
    let interfaces = || protocols().flat_map(|p| p.interfaces.iter().map(move |i| (p, i)));
    let mut interfaces: Vec<_> = interfaces().collect();
    interfaces.sort_by_key(|(_, i)| &i.name);
    wl!("#![allow(non_upper_case_globals, unused)]")?;
    wl!()?;
    wl!("use linearize::{{StaticCopyMap, Linearize}};")?;
    wl!("use crate::protocols::ObjectInterface;")?;
    wl!()?;
    for (_, interface) in &interfaces {
        let version = interface.version;
        let snake = &interface.name;
        wl!(r#"const {snake}: u32 = {version};"#)?;
    }
    wl!()?;
    wl!("#[rustfmt::skip]")?;
    wl!("pub(in super::super) const BASELINE: &StaticCopyMap<ObjectInterface, u32> = {{")?;
    wl!("    static BASELINE: [u32; ObjectInterface::LENGTH] = {{")?;
    wl!("        let mut baseline = [0; ObjectInterface::LENGTH];")?;
    for (protocol, interface) in &interfaces {
        let snake = &interface.name;
        let camel = format_camel(snake);
        if protocol.is_wlproxy_test {
            wl!(r#"        #[cfg(test)]"#)?;
        } else if !protocol.is_wayland {
            wl!(r#"        #[cfg(feature = "protocol-{}")]"#, protocol.name)?;
        }
        wl!(
            r#"        {{ baseline[ObjectInterface::{camel}.__linearize_d66aa8fa_6974_4651_b2b7_75291a9e7105()] = {snake}; }}"#
        )?;
    }
    wl!("        baseline")?;
    wl!("    }};")?;
    wl!("    StaticCopyMap::from_ref(&BASELINE)")?;
    wl!("}};")?;
    Ok(())
}

pub fn format_baseline_txt(w: &mut impl Write, suits: &[Suite]) -> io::Result<()> {
    define_w!(w);
    let mut interfaces: Vec<_> = suits
        .iter()
        .flat_map(|s| s.protocols.iter())
        .flat_map(|p| p.interfaces.iter().map(move |i| (p, i)))
        .collect();
    interfaces.sort_by_key(|(_, i)| &i.name);
    for (_, interface) in &interfaces {
        let version = interface.version;
        let snake = &interface.name;
        wl!(r#"{snake} = {version}"#)?;
    }
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
    wl!("#![allow(clippy::needless_return)]")?;
    wl!("#![allow(clippy::manual_div_ceil)]")?;
    wl!("#![allow(clippy::match_single_binding)]")?;
    wl!("#![allow(clippy::collapsible_if)]")?;
    wl!("#![allow(clippy::doc_overindented_list_items)]")?;
    wl!("#![allow(unused_imports)]")?;
    wl!("#![allow(non_snake_case)]")?;
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

fn format_message_doc<W>(w: &mut W, outgoing: bool, new: bool, message: &Message) -> io::Result<()>
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
    if outgoing {
        if new {
            for arg in &message.args {
                if arg.ty == ArgType::NewId {
                    num_args -= 1;
                    break;
                }
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
            if new && arg.ty == ArgType::NewId {
                continue;
            }
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
            let mut spaces = 0usize;
            'spaces: {
                for c in line.chars() {
                    if c == ' ' {
                        spaces += 1;
                    } else if c == '\t' {
                        spaces = (spaces + 8) & !7;
                    } else {
                        break 'spaces;
                    }
                }
                continue 'outer;
            }
            trim = Some(spaces);
        }
        let trim = trim.unwrap();
        let mut line_buf = String::new();
        // let mut line = line;
        if line.contains('\t') {
            let mut offset = 0;
            for c in line.chars() {
                if c == '\t' {
                    line_buf.push(' ');
                    offset += 1;
                    let delta = (-offset) & 7;
                    for _ in 0..delta {
                        line_buf.push(' ');
                    }
                    offset += delta;
                } else {
                    line_buf.push(c);
                    offset += 1;
                }
            }
            line = &line_buf;
        }
        let idx = 'idx: {
            let mut spaces = 0usize;
            for (idx, c) in line.char_indices() {
                if c == ' ' {
                    spaces += 1;
                } else {
                    break 'idx idx;
                }
                if spaces >= trim {
                    break 'idx idx + 1;
                }
            }
            line.len()
        };
        line = &line[idx..];
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

fn format_object_impl(w: &mut impl Write, interface: &Interface) -> io::Result<()> {
    define_w!(w);
    let snake = &interface.name;
    let camel = format_camel(snake).to_string();
    wl!(r#"impl ObjectPrivate for {PREFIX}{camel} {{"#)?;
    wl!(r#"    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {{"#)?;
    wl!(r#"        Rc::<Self>::new_cyclic(|slf| Self {{"#)?;
    wl!(
        r#"            core: ObjectCore::new(state, slf.clone(), ObjectInterface::{camel}, version),"#
    )?;
    wl!(r#"            handler: Default::default(),"#)?;
    if interface.is_wl_registry {
        wl!(r#"            names: Default::default(),"#)?;
    }
    wl!(r#"        }})"#)?;
    wl!(r#"    }}"#)?;
    wl!()?;
    wl!(r#"    fn delete_id(self: Rc<Self>) -> Result<(), (ObjectError, Rc<dyn Object>)> {{"#)?;
    wl!(r#"        let Some(mut handler) = self.handler.try_borrow_mut() else {{"#)?;
    wl!(r#"            return Err((ObjectError(ObjectErrorKind::HandlerBorrowed), self));"#)?;
    wl!(r#"        }};"#)?;
    wl!(r#"        if let Some(handler) = &mut *handler {{"#)?;
    wl!(r#"            handler.delete_id(&self);"#)?;
    wl!(r#"        }} else {{"#)?;
    wl!(r#"            self.core.delete_id();"#)?;
    wl!(r#"        }}"#)?;
    wl!(r#"        Ok(())"#)?;
    wl!(r#"    }}"#)?;
    wl!()?;
    wl!(
        r#"    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {{"#
    )?;
    format_object_message_handler_body(w, interface, true)?;
    wl!(r#"    }}"#)?;
    wl!()?;
    wl!(
        r#"    fn handle_event(self: Rc<Self>, server: &Endpoint, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {{"#
    )?;
    format_object_message_handler_body(w, interface, false)?;
    wl!(r#"    }}"#)?;
    wl!()?;
    wl!(r#"    fn get_request_name(&self, id: u32) -> Option<&'static str> {{"#)?;
    format_object_message_name(w, interface, true)?;
    wl!(r#"    }}"#)?;
    wl!()?;
    wl!(r#"    fn get_event_name(&self, id: u32) -> Option<&'static str> {{"#)?;
    format_object_message_name(w, interface, false)?;
    wl!(r#"    }}"#)?;
    wl!(r#"}}"#)?;
    wl!()?;
    wl!(r#"impl Object for {PREFIX}{camel} {{"#)?;
    wl!(r#"    fn core(&self) -> &ObjectCore {{"#)?;
    wl!(r#"        &self.core"#)?;
    wl!(r#"    }}"#)?;
    wl!()?;
    wl!(r#"    fn unset_handler(&self) {{"#)?;
    wl!(r#"        self.handler.set(None);"#)?;
    wl!(r#"    }}"#)?;
    wl!()?;
    wl!(
        r#"    fn get_handler_any_ref(&self) -> Result<HandlerRef<'_, dyn Any>, HandlerAccessError> {{"#
    )?;
    format_object_get_handler(w, false)?;
    wl!(r#"    }}"#)?;
    wl!()?;
    wl!(
        r#"    fn get_handler_any_mut(&self) -> Result<HandlerMut<'_, dyn Any>, HandlerAccessError> {{"#
    )?;
    format_object_get_handler(w, true)?;
    wl!(r#"    }}"#)?;
    wl!(r#"}}"#)?;
    Ok(())
}

fn format_object_get_handler(w: &mut impl Write, mutable: bool) -> io::Result<()> {
    define_w!(w);
    let p = "        ";
    let mut suffix = "";
    if mutable {
        suffix = "_mut";
    }
    wl!(
        r#"{p}let borrowed = self.handler.try_borrow{suffix}().ok_or(HandlerAccessError::AlreadyBorrowed)?;"#
    )?;
    wl!(r#"{p}if borrowed.is_none() {{"#)?;
    wl!(r#"{p}    return Err(HandlerAccessError::NoHandler);"#)?;
    wl!(r#"{p}}}"#)?;
    if mutable {
        wl!(
            r#"{p}Ok(HandlerMut::map(borrowed, |handler| &mut **handler.as_mut().unwrap() as &mut dyn Any))"#
        )?;
    } else {
        wl!(
            r#"{p}Ok(HandlerRef::map(borrowed, |handler| &**handler.as_ref().unwrap() as &dyn Any))"#
        )?;
    }
    Ok(())
}

fn format_object_message_name(
    w: &mut impl Write,
    interface: &Interface,
    requests: bool,
) -> io::Result<()> {
    define_w!(w);
    let p = "        ";
    if interface.messages.iter().any(|m| m.is_request == requests) {
        wl!(r#"{p}let name = match id {{"#)?;
        for msg in &interface.messages {
            if msg.is_request != requests {
                continue;
            }
            wl!(r#"{p}    {} => "{}","#, msg.message_id, msg.name)?;
        }
        wl!(r#"{p}    _ => return None,"#)?;
        wl!(r#"{p}}};"#)?;
        wl!(r#"{p}Some(name)"#)?;
    } else {
        wl!(r#"{p}let _ = id;"#)?;
        wl!(r#"{p}None"#)?;
    }
    Ok(())
}

fn format_wayland_debug(
    w: &mut impl Write,
    interface: &Interface,
    msg: &Message,
    outgoing: bool,
) -> io::Result<()> {
    define_w!(w);
    let mut prefix = "        ";
    if !outgoing {
        prefix = "                ";
    }
    wl!(r#"{prefix}#[cfg(feature = "logging")]"#)?;
    wl!(r#"{prefix}if self.core.state.log {{"#)?;
    wl!(r#"{prefix}    #[cold]"#)?;
    w!(r#"{prefix}    fn log(state: &State"#)?;
    if msg.is_request ^ outgoing {
        w!(r#", client_id: u64"#)?;
    }
    w!(r#", id: u32"#)?;
    for (idx, arg) in msg.args.iter().enumerate() {
        let ty = match arg.ty {
            ArgType::NewId => {
                if arg.interface.is_none() {
                    w!(r#", arg{idx}_interface: &str"#)?;
                    w!(r#", arg{idx}_id: u32"#)?;
                    w!(r#", arg{idx}_version: u32"#)?;
                    continue;
                }
                "u32"
            }
            ArgType::Object => "u32",
            ArgType::Uint | ArgType::Int if arg.enum_.is_some() => {
                w!(r#", arg{idx}: {}"#, arg_type(interface, arg))?;
                continue;
            }
            ArgType::Int => "i32",
            ArgType::Uint => "u32",
            ArgType::Fixed => "Fixed",
            ArgType::String if arg.allow_null => "Option<&str>",
            ArgType::String => "&str",
            ArgType::Array => "&[u8]",
            ArgType::Fd => "i32",
        };
        w!(r#", arg{idx}: {ty}"#)?;
    }
    wl!(r#") {{"#)?;
    wl!(r#"{prefix}        let (millis, micros) = time_since_epoch();"#)?;
    wl!(r#"{prefix}        let prefix = &state.log_prefix;"#)?;
    w!(r#"{prefix}        let args = format_args!("[{{millis:7}}.{{micros:03}}] "#)?;
    w!(r#"{{prefix}}"#)?;
    if msg.is_request ^ outgoing {
        w!(r#"client#{{:<4}}"#)?;
    } else {
        w!(r#"server     "#)?;
    }
    if outgoing {
        w!(r#" <= "#)?;
    } else {
        w!(r#" -> "#)?;
    }
    w!(r#"{}#{{}}.{}("#, interface.name, msg.name)?;
    for (idx, arg) in msg.args.iter().enumerate() {
        if idx > 0 {
            w!(r#", "#)?;
        }
        w!(r#"{}: "#, arg.name)?;
        match arg.ty {
            ArgType::NewId | ArgType::Object => {
                if let Some(interface) = &arg.interface {
                    w!(r#"{interface}"#)?
                } else if arg.ty == ArgType::NewId {
                    w!(r#"{{}}"#)?
                } else {
                    w!(r#"unknown"#)?
                }
                w!(r#"#{{}}"#)?;
                if arg.interface.is_none() && arg.ty == ArgType::NewId {
                    w!(r#" (version: {{}})"#)?;
                }
            }
            _ if arg.enum_.is_some() => w!(r#"{{:?}}"#)?,
            ArgType::Int | ArgType::Uint | ArgType::Fixed | ArgType::Fd | ArgType::Array => {
                w!(r#"{{}}"#)?
            }
            ArgType::String => w!(r#"{{:?}}"#)?,
        }
    }
    w!(r#")\n""#)?;
    if msg.is_request ^ outgoing {
        w!(r#", client_id"#)?;
    }
    w!(r#", id"#)?;
    for (idx, arg) in msg.args.iter().enumerate() {
        match arg.ty {
            ArgType::NewId => {
                if arg.interface.is_none() {
                    w!(r#", arg{idx}_interface, arg{idx}_id, arg{idx}_version"#)?;
                } else {
                    w!(r#", arg{idx}"#)?;
                }
            }
            ArgType::Array => w!(r#", debug_array(arg{idx})"#)?,
            _ => w!(r#", arg{idx}"#)?,
        }
    }
    wl!(r#");"#)?;
    wl!(r#"{prefix}        state.log(args);"#)?;
    wl!(r#"{prefix}    }}"#)?;
    w!(r#"{prefix}    log(&self.core.state"#)?;
    if msg.is_request ^ outgoing {
        w!(r#", client.endpoint.id"#)?;
    }
    if outgoing {
        w!(r#", id"#)?;
    } else {
        w!(r#", msg[0]"#)?;
    }
    for (idx, arg) in msg.args.iter().enumerate() {
        match arg.ty {
            ArgType::NewId => {
                if arg.interface.is_none() {
                    if outgoing {
                        w!(r#", arg{idx}.interface.name()"#)?;
                    } else {
                        w!(r#", arg{idx}_interface"#)?;
                    }
                }
                if outgoing {
                    w!(r#", arg{idx}_id"#)?;
                } else {
                    w!(r#", arg{idx}"#)?;
                }
                if arg.interface.is_none() {
                    if outgoing {
                        w!(r#", arg{idx}.version"#)?;
                    } else {
                        w!(r#", arg{idx}_version"#)?;
                    }
                }
            }
            ArgType::Object => {
                if outgoing {
                    w!(r#", arg{idx}_id"#)?
                } else {
                    w!(r#", arg{idx}"#)?
                }
            }
            ArgType::Int | ArgType::Uint | ArgType::Fixed | ArgType::String => w!(r#", arg{idx}"#)?,
            ArgType::Array => w!(r#", arg{idx}"#)?,
            ArgType::Fd => w!(r#", arg{idx}.as_raw_fd()"#)?,
        }
    }
    wl!(r#");"#)?;
    wl!(r#"{prefix}}}"#)?;
    Ok(())
}

fn format_object_message_handler_body<W: Write>(
    w: &mut W,
    interface: &Interface,
    requests: bool,
) -> io::Result<()> {
    define_w!(w);
    let p = "        ";
    wl!(r#"{p}let Some(mut handler) = self.handler.try_borrow_mut() else {{"#)?;
    wl!(r#"{p}    return Err(ObjectError(ObjectErrorKind::HandlerBorrowed));"#)?;
    wl!(r#"{p}}};"#)?;
    wl!(r#"{p}let handler = &mut *handler;"#)?;
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
            wl!(
                r#"{p}            return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, {})));"#,
                (num_words + 2) * 4
            )?;
            wl!(r#"{p}        }};"#)?;
        } else if msg.args.iter().any(|a| !matches!(a.ty, ArgType::Fd)) {
            wl!(r#"{p}        let mut offset = 2;"#)?;
            for (idx, arg) in msg.args.iter().enumerate() {
                match arg.ty {
                    ArgType::NewId
                    | ArgType::Int
                    | ArgType::Uint
                    | ArgType::Fixed
                    | ArgType::Object => {
                        if arg.ty == ArgType::NewId && arg.interface.is_none() {
                            wl!(r#"{p}        let arg{idx}_interface;"#)?;
                            wl!(
                                r#"{p}        (arg{idx}_interface, offset) = parse_string::<NonNullString>(msg, offset, "{}")?;"#,
                                arg.name,
                            )?;
                            wl!(
                                r#"{p}        let Some(&arg{idx}_version) = msg.get(offset) else {{"#
                            )?;
                            wl!(
                                r#"{p}            return Err(ObjectError(ObjectErrorKind::MissingArgument("{}")));"#,
                                arg.name
                            )?;
                            wl!(r#"{p}        }};"#)?;
                            wl!(r#"{p}        offset += 1;"#)?;
                        }
                        wl!(r#"{p}        let Some(&arg{idx}) = msg.get(offset) else {{"#)?;
                        wl!(
                            r#"{p}            return Err(ObjectError(ObjectErrorKind::MissingArgument("{}")));"#,
                            arg.name
                        )?;
                        wl!(r#"{p}        }};"#)?;
                        wl!(r#"{p}        offset += 1;"#)?;
                    }
                    ArgType::Fd => continue,
                    ArgType::String => {
                        let str_type = match arg.allow_null {
                            true => "NullableString",
                            false => "NonNullString",
                        };
                        wl!(r#"{p}        let arg{idx};"#)?;
                        wl!(
                            r#"{p}        (arg{idx}, offset) = parse_string::<{str_type}>(msg, offset, "{}")?;"#,
                            arg.name,
                        )?;
                    }
                    ArgType::Array => {
                        wl!(r#"{p}        let arg{idx};"#)?;
                        wl!(
                            r#"{p}        (arg{idx}, offset) = parse_array(msg, offset, "{}")?;"#,
                            arg.name,
                        )?;
                    }
                }
            }
            wl!(r#"{p}        if offset != msg.len() {{"#)?;
            wl!(r#"{p}            return Err(ObjectError(ObjectErrorKind::TrailingBytes));"#)?;
            wl!(r#"{p}        }}"#)?;
        } else {
            wl!(r#"{p}        if msg.len() != 2 {{"#)?;
            wl!(
                r#"{p}            return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 8)));"#
            )?;
            wl!(r#"{p}        }}"#)?;
        }
        for (idx, arg) in msg.args.iter().enumerate() {
            if arg.ty == ArgType::Fd {
                wl!(r#"{p}        let Some(arg{idx}) = fds.pop_front() else {{"#)?;
                wl!(
                    r#"{p}            return Err(ObjectError(ObjectErrorKind::MissingFd("{}")));"#,
                    arg.name
                )?;
                wl!(r#"{p}        }};"#)?;
            }
        }
        for (idx, arg) in msg.args.iter().enumerate() {
            match arg.ty {
                ArgType::String | ArgType::Array => continue,
                ArgType::Uint if arg.enum_.is_none() => continue,
                ArgType::NewId | ArgType::Object => continue,
                _ => {}
            }
            w!(r#"                let arg{idx} = "#)?;
            match arg.ty {
                ArgType::Int | ArgType::Uint if arg.enum_.is_some() => {
                    w!(r#"{}(arg{idx})"#, arg_type(interface, arg))?;
                }
                ArgType::Int => {
                    w!(r#"arg{idx} as i32"#)?;
                }
                ArgType::NewId
                | ArgType::Object
                | ArgType::Uint
                | ArgType::String
                | ArgType::Array => {
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
        format_wayland_debug(w, interface, msg, false)?;
        if interface.is_wl_registry && msg.name == "global" {
            wl!(r#"{p}        let Some(arg1) = ObjectInterface::from_str(arg1) else {{"#)?;
            wl!(r#"{p}            return Ok(());"#)?;
            wl!(r#"{p}        }};"#)?;
            wl!(r#"{p}        let max_version = self.core.state.baseline.1[arg1];"#)?;
            wl!(r#"{p}        if max_version == 0 {{"#)?;
            wl!(r#"{p}            return Ok(());"#)?;
            wl!(r#"{p}        }}"#)?;
            wl!(r#"{p}        self.names.borrow_mut().insert(arg0);"#)?;
            wl!(r#"{p}        let arg2 = max_version.min(arg2);"#)?;
        }
        if interface.is_wl_registry && msg.name == "global_remove" {
            wl!(r#"{p}        if !self.names.borrow_mut().remove(&arg0) {{"#)?;
            wl!(r#"{p}            return Ok(());"#)?;
            wl!(r#"{p}        }}"#)?;
        }
        for (idx, arg) in msg.args.iter().enumerate() {
            match arg.ty {
                ArgType::NewId => {
                    wl!(r#"{p}        let arg{idx}_id = arg{idx};"#)?;
                    match &arg.interface {
                        Some(interface) => {
                            let camel = format_camel(interface);
                            wl!(
                                r#"{p}        let arg{idx} = {PREFIX}{camel}::new(&self.core.state, self.core.version);"#
                            )?;
                        }
                        _ => {
                            wl!(
                                r#"{p}        let arg{idx} = create_object_for_interface(&self.core.state, arg{idx}_interface, arg{idx}_version)?;"#
                            )?;
                        }
                    }
                    if msg.is_request {
                        wl!(
                            r#"{p}        arg{idx}.core().set_client_id(client, arg{idx}_id, arg{idx}.clone())"#
                        )?;
                        wl!(
                            r#"{p}            .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg{idx}_id, "{}", e)))?;"#,
                            arg.name,
                        )?;
                    } else {
                        wl!(
                            r#"{p}        arg{idx}.core().set_server_id(arg{idx}_id, arg{idx}.clone())"#
                        )?;
                        wl!(
                            r#"{p}            .map_err(|e| ObjectError(ObjectErrorKind::SetServerId(arg{idx}_id, "{}", e)))?;"#,
                            arg.name,
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
                    wl!(r#"{p}        {prefix}let arg{idx}_id = arg{idx};"#)?;
                    if msg.is_request {
                        wl!(
                            r#"{p}        {prefix}let Some(arg{idx}) = client.endpoint.lookup(arg{idx}_id) else {{"#
                        )?;
                        wl!(
                            r#"{p}            {prefix}return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg{idx}_id)));"#
                        )?;
                    } else {
                        wl!(
                            r#"{p}        {prefix}let Some(arg{idx}) = server.lookup(arg{idx}_id) else {{"#
                        )?;
                        wl!(
                            r#"{p}            {prefix}return Err(ObjectError(ObjectErrorKind::NoServerObject(arg{idx}_id)));"#
                        )?;
                    }
                    wl!(r#"{p}        {prefix}}};"#)?;
                    if let Some(interface) = &arg.interface {
                        let camel = format_camel(interface);
                        wl!(
                            r#"{p}        {prefix}let Ok(arg{idx}) = (arg{idx} as Rc<dyn Any>).downcast::<{PREFIX}{camel}>() else {{"#
                        )?;
                        if msg.is_request {
                            wl!(
                                r#"{p}        {prefix}    let o = client.endpoint.lookup(arg{idx}_id).unwrap();"#
                            )?;
                        } else {
                            wl!(
                                r#"{p}        {prefix}    let o = server.lookup(arg{idx}_id).unwrap();"#
                            )?;
                        }
                        wl!(
                            r#"{p}        {prefix}    return Err(ObjectError(ObjectErrorKind::WrongObjectType("{}", o.core().interface, ObjectInterface::{camel})));"#,
                            arg.name
                        )?;
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
                ArgType::NewId | ArgType::Object if arg.interface.is_some() => {}
                _ => continue,
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
                _ => {
                    unreachable!();
                }
            }
            wl!(r#";"#)?;
        }
        if interface.is_wl_display && !msg.is_request {
            if msg.name == "delete_id" {
                wl!(r#"{p}        self.core.state.handle_delete_id(server, arg0);"#)?;
            } else if msg.name == "error" {
                wl!(
                    r#"{p}        return Err(ObjectError(ObjectErrorKind::ServerError(arg0.core().interface, arg0_id, arg1, StringError(arg2.to_string()))));"#
                )?;
            }
        } else {
            macro_rules! format_call {
                ($target:expr) => {
                    w!(r#"{p}            {}.handle_{}("#, $target, msg.name)?;
                    w!(r#"&self"#)?;
                    for (idx, _) in msg.args.iter().enumerate() {
                        w!(r#", arg{idx}"#)?;
                    }
                    wl!(r#");"#)?;
                };
            }
            if msg.ty == Some(MessageType::Destructor) {
                if msg.is_request {
                    wl!(r#"{p}        self.core.handle_client_destroy();"#)?;
                } else {
                    wl!(r#"{p}        self.core.handle_server_destroy();"#)?;
                }
            }
            if interface.is_wl_fixes && msg.name == "destroy_registry" {
                wl!(r#"{p}        arg0.core().handle_client_destroy();"#)?;
            }
            wl!(r#"{p}        if let Some(handler) = handler {{"#)?;
            format_call!("(**handler)");
            wl!(r#"{p}        }} else {{"#)?;
            format_call!("DefaultHandler");
            wl!(r#"{p}        }}"#)?;
        }
        wl!(r#"{p}    }}"#)?;
    }
    wl!(r#"{p}    n => {{"#)?;
    if requests {
        wl!(r#"{p}        let _ = client;"#)?;
    } else {
        wl!(r#"{p}        let _ = server;"#)?;
    }
    wl!(r#"{p}        let _ = msg;"#)?;
    wl!(r#"{p}        let _ = fds;"#)?;
    wl!(r#"{p}        let _ = handler;"#)?;
    wl!(r#"{p}        return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));"#)?;
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
    if interface.enums.is_not_empty() {
        wl!(r#"impl {PREFIX}{camel} {{"#)?;
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
        wl!(r#"pub struct {PREFIX}{camel}(pub u32);"#)?;
        if enum_.bitfield {
            wl!()?;
            wl!(r#"/// An iterator over the set bits in a [`{PREFIX}{camel}`]."#)?;
            wl!(r#"///"#)?;
            wl!(
                r#"/// You can construct this with the `IntoIterator` implementation of `{PREFIX}{camel}`."#
            )?;
            wl!(r#"#[derive(Clone, Debug)]"#)?;
            wl!(r#"pub struct {PREFIX}{camel}Iter(pub u32);"#)?;
        }
        if enum_.entries.is_not_empty() {
            wl!()?;
            wl!(r#"impl {PREFIX}{camel} {{"#)?;
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
            wl!(r#"impl {PREFIX}{camel} {{"#)?;
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
            wl!(r#"impl Iterator for {PREFIX}{camel}Iter {{"#)?;
            wl!(r#"    type Item = {PREFIX}{camel};"#)?;
            wl!()?;
            wl!(r#"    fn next(&mut self) -> Option<Self::Item> {{"#)?;
            wl!(r#"        if self.0 == 0 {{"#)?;
            wl!(r#"            return None;"#)?;
            wl!(r#"        }}"#)?;
            wl!(r#"        let bit = 1 << self.0.trailing_zeros();"#)?;
            wl!(r#"        self.0 &= !bit;"#)?;
            wl!(r#"        Some({PREFIX}{camel}(bit))"#)?;
            wl!(r#"    }}"#)?;
            wl!(r#"}}"#)?;
            wl!()?;
            wl!(r#"impl IntoIterator for {PREFIX}{camel} {{"#)?;
            wl!(r#"    type Item = {PREFIX}{camel};"#)?;
            wl!(r#"    type IntoIter = {PREFIX}{camel}Iter;"#)?;
            wl!()?;
            wl!(r#"    fn into_iter(self) -> Self::IntoIter {{"#)?;
            wl!(r#"        {PREFIX}{camel}Iter(self.0)"#)?;
            wl!(r#"    }}"#)?;
            wl!(r#"}}"#)?;
            macro_rules! bitop {
                ($capital:literal, $lower:literal, $op:literal) => {{
                    wl!()?;
                    wl!(r#"impl Bit{} for {PREFIX}{camel} {{"#, $capital)?;
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
                    wl!(r#"impl Bit{}Assign for {PREFIX}{camel} {{"#, $capital)?;
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
            wl!(r#"impl Sub for {PREFIX}{camel} {{"#)?;
            wl!(r#"    type Output = Self;"#)?;
            wl!()?;
            wl!(r#"    fn sub(self, rhs: Self) -> Self::Output {{"#)?;
            wl!(r#"        self.difference(rhs)"#)?;
            wl!(r#"    }}"#)?;
            wl!(r#"}}"#)?;
            wl!()?;
            wl!(r#"impl SubAssign for {PREFIX}{camel} {{"#)?;
            wl!(r#"    fn sub_assign(&mut self, rhs: Self) {{"#)?;
            wl!(r#"        *self = self.difference(rhs);"#)?;
            wl!(r#"    }}"#)?;
            wl!(r#"}}"#)?;
            wl!()?;
            wl!(r#"impl Not for {PREFIX}{camel} {{"#)?;
            wl!(r#"    type Output = Self;"#)?;
            wl!()?;
            wl!(r#"    fn not(self) -> Self::Output {{"#)?;
            wl!(r#"        self.complement()"#)?;
            wl!(r#"    }}"#)?;
            wl!(r#"}}"#)?;
        }
        wl!()?;
        wl!(r#"impl Debug for {PREFIX}{camel} {{"#)?;
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
    wl!(r#"impl Debug for {PREFIX}{camel} {{"#)?;
    wl!(r#"    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {{"#)?;
    wl!(r#"        f.debug_struct("{PREFIX}{camel}")"#)?;
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
