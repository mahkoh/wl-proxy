use {
    crate::{
        HfError,
        libdrm::{get_modifier_name, get_modifier_vendor, get_nodes},
    },
    bstr::ByteSlice,
    error_reporter::Report,
    memmap2::{Mmap, MmapOptions},
    std::{fmt, mem, os::fd::OwnedFd, process::Command, rc::Rc},
    uapi::c::dev_t,
    wl_proxy::{
        baseline::Baseline,
        object::{ConcreteObject, Object, ObjectCoreApi, ObjectRcUtils},
        protocols::{
            linux_dmabuf_v1::{
                zwp_linux_dmabuf_feedback_v1::{
                    ZwpLinuxDmabufFeedbackV1, ZwpLinuxDmabufFeedbackV1Handler,
                    ZwpLinuxDmabufFeedbackV1TrancheFlags,
                },
                zwp_linux_dmabuf_v1::{ZwpLinuxDmabufV1, ZwpLinuxDmabufV1Handler},
            },
            wayland::{
                wl_display::{WlDisplay, WlDisplayHandler},
                wl_registry::{WlRegistry, WlRegistryHandler},
                wl_surface::WlSurface,
            },
        },
        simple::{SimpleCommandExt, SimpleProxy},
    },
};

pub fn main(program: Vec<String>) -> Result<(), HfError> {
    let server = SimpleProxy::new(Baseline::ALL_OF_THEM).map_err(HfError::CreateServer)?;
    Command::new(&program[0])
        .args(&program[1..])
        .with_wayland_display(server.display())
        .spawn_and_forward_exit_code()
        .map_err(HfError::SpawnChild)?;
    let err = server.run(|| WlDisplayHandlerImpl);
    Err(HfError::ServerFailed(err))
}

struct WlDisplayHandlerImpl;

impl WlDisplayHandler for WlDisplayHandlerImpl {
    fn handle_get_registry(&mut self, slf: &Rc<WlDisplay>, registry: &Rc<WlRegistry>) {
        registry.set_handler(WlRegistryHandlerImpl);
        slf.send_get_registry(registry);
    }
}

struct WlRegistryHandlerImpl;

impl WlRegistryHandler for WlRegistryHandlerImpl {
    fn handle_bind(&mut self, slf: &Rc<WlRegistry>, name: u32, id: Rc<dyn Object>) {
        if id.interface() == ZwpLinuxDmabufV1::INTERFACE {
            id.downcast::<ZwpLinuxDmabufV1>()
                .set_handler(ZwpLinuxDmabufV1HandlerImpl);
        }
        slf.send_bind(name, id);
    }
}

struct ZwpLinuxDmabufV1HandlerImpl;

impl ZwpLinuxDmabufV1Handler for ZwpLinuxDmabufV1HandlerImpl {
    fn handle_get_default_feedback(
        &mut self,
        slf: &Rc<ZwpLinuxDmabufV1>,
        id: &Rc<ZwpLinuxDmabufFeedbackV1>,
    ) {
        id.set_handler(ZwpLinuxDmabufFeedbackV1HandlerImpl {
            fb_id: id.client_id().unwrap(),
            surface_id: Default::default(),
            table: Default::default(),
            main_device: Default::default(),
            tranches: Default::default(),
            tranche: Default::default(),
        });
        slf.send_get_default_feedback(id);
    }

    fn handle_get_surface_feedback(
        &mut self,
        slf: &Rc<ZwpLinuxDmabufV1>,
        id: &Rc<ZwpLinuxDmabufFeedbackV1>,
        surface: &Rc<WlSurface>,
    ) {
        id.set_handler(ZwpLinuxDmabufFeedbackV1HandlerImpl {
            fb_id: id.client_id().unwrap(),
            surface_id: surface.client_id(),
            table: Default::default(),
            main_device: Default::default(),
            tranches: Default::default(),
            tranche: Default::default(),
        });
        slf.send_get_surface_feedback(id, surface);
    }
}

struct ZwpLinuxDmabufFeedbackV1HandlerImpl {
    fb_id: u32,
    surface_id: Option<u32>,
    table: Option<Mmap>,

    main_device: Option<dev_t>,
    tranches: Vec<Tranche>,
    tranche: Tranche,
}

#[derive(Default)]
struct Tranche {
    device: Option<dev_t>,
    flags: Option<ZwpLinuxDmabufFeedbackV1TrancheFlags>,
    formats: Vec<(u32, u64)>,
}

impl ZwpLinuxDmabufFeedbackV1Handler for ZwpLinuxDmabufFeedbackV1HandlerImpl {
    fn handle_done(&mut self, slf: &Rc<ZwpLinuxDmabufFeedbackV1>) {
        slf.send_done();
        if let Some(id) = self.surface_id {
            eprint!("feedback for wl_surface#{id}")
        } else {
            eprint!("default feedback")
        }
        eprintln!(" (zwp_linux_dmabuf_feedback_v1#{})", self.fb_id);
        let print_nodes = |dev: dev_t| {
            let nodes = get_nodes(dev);
            fmt::from_fn(move |fmt| {
                if nodes.is_empty() {
                    fmt.write_str("device has no nodes")
                } else {
                    for (idx, node) in nodes.iter().enumerate() {
                        if idx > 0 {
                            fmt.write_str(" or ")?;
                        }
                        fmt.write_str("/dev/dri/")?;
                        fmt.write_str(node)?;
                    }
                    Ok(())
                }
            })
        };
        if let Some(id) = self.main_device.take() {
            eprintln!("    main device: 0x{id:X} ({})", print_nodes(id));
        }
        for tranche in mem::take(&mut self.tranches) {
            eprintln!("    tranche");
            if let Some(id) = tranche.device {
                eprintln!("        target device: 0x{id:X} ({})", print_nodes(id));
            }
            if let Some(flags) = tranche.flags {
                eprintln!("        flags: {flags:?}");
            }
            eprintln!("        formats (fourcc) and modifiers (names):");
            for &(format, modifier) in &tranche.formats {
                let format_name = format.to_le_bytes();
                let format_name = format_name.as_bstr();
                eprint!("        0x{format:08x} = '{format_name: <4}'; 0x{modifier:016x} = ");
                let Some(vendor) = get_modifier_vendor(modifier) else {
                    eprintln!("UNKNOWN");
                    continue;
                };
                let Some(name) = get_modifier_name(modifier) else {
                    eprintln!("{vendor}_UNKNOWN_MODIFIER");
                    continue;
                };
                if modifier >> 56 == 0 {
                    eprintln!("{name}");
                } else {
                    eprintln!("{vendor}_{name}");
                }
            }
        }
    }

    fn handle_format_table(
        &mut self,
        slf: &Rc<ZwpLinuxDmabufFeedbackV1>,
        fd: &Rc<OwnedFd>,
        size: u32,
    ) {
        slf.send_format_table(fd, size);
        self.table = None;
        let table = unsafe { MmapOptions::default().map_copy_read_only(fd) };
        match table {
            Ok(t) => self.table = Some(t),
            Err(e) => {
                log::error!("Failed to map table: {:?}", Report::new(e));
            }
        }
    }

    fn handle_main_device(&mut self, slf: &Rc<ZwpLinuxDmabufFeedbackV1>, device: &[u8]) {
        slf.send_main_device(device);
        let dev = uapi::pod_read::<dev_t, _>(device).unwrap();
        self.main_device = Some(dev);
    }

    fn handle_tranche_done(&mut self, slf: &Rc<ZwpLinuxDmabufFeedbackV1>) {
        slf.send_tranche_done();
        self.tranches.push(mem::take(&mut self.tranche));
    }

    fn handle_tranche_target_device(&mut self, slf: &Rc<ZwpLinuxDmabufFeedbackV1>, device: &[u8]) {
        slf.send_tranche_target_device(device);
        let dev = uapi::pod_read::<dev_t, _>(device).unwrap();
        self.tranche.device = Some(dev);
    }

    fn handle_tranche_formats(&mut self, slf: &Rc<ZwpLinuxDmabufFeedbackV1>, indices: &[u8]) {
        slf.send_tranche_formats(indices);
        let table = self.table.as_ref().unwrap();
        for index in uapi::pod_iter::<u16, _>(indices).unwrap() {
            let format = uapi::pod_read_init::<u32, _>(&table[index as usize * 16..]).unwrap();
            let modifier =
                uapi::pod_read_init::<u64, _>(&table[index as usize * 16 + 8..]).unwrap();
            self.tranche.formats.push((format, modifier));
        }
    }

    fn handle_tranche_flags(
        &mut self,
        slf: &Rc<ZwpLinuxDmabufFeedbackV1>,
        flags: ZwpLinuxDmabufFeedbackV1TrancheFlags,
    ) {
        slf.send_tranche_flags(flags);
        self.tranche.flags = Some(flags);
    }
}
