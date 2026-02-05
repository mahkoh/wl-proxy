use {
    crate::{HfError, cli::Filter},
    error_reporter::Report,
    memmap2::{Mmap, MmapOptions},
    std::{collections::HashMap, os::fd::OwnedFd, process::Command, rc::Rc, sync::Arc},
    wl_proxy::{
        baseline::Baseline,
        object::{ConcreteObject, Object, ObjectCoreApi, ObjectRcUtils},
        protocols::{
            drm::wl_drm::{WlDrm, WlDrmHandler},
            linux_dmabuf_v1::{
                zwp_linux_dmabuf_feedback_v1::{
                    ZwpLinuxDmabufFeedbackV1, ZwpLinuxDmabufFeedbackV1Handler,
                },
                zwp_linux_dmabuf_v1::{ZwpLinuxDmabufV1, ZwpLinuxDmabufV1Handler},
            },
            wayland::{
                wl_display::{WlDisplay, WlDisplayHandler},
                wl_registry::{WlRegistry, WlRegistryHandler},
                wl_shm::{WlShm, WlShmFormat, WlShmHandler},
                wl_surface::WlSurface,
            },
        },
        simple::{SimpleCommandExt, SimpleProxy},
    },
};

pub fn main(allow: Vec<Filter>, deny: Vec<Filter>, program: Vec<String>) -> Result<(), HfError> {
    let server = SimpleProxy::new(Baseline::V2).map_err(HfError::CreateServer)?;
    Command::new(&program[0])
        .args(&program[1..])
        .with_wayland_display(server.display())
        .spawn_and_forward_exit_code()
        .map_err(HfError::SpawnChild)?;
    let dispositions = create_disposition_list(&allow, &deny);
    let err = server.run(|| WlDisplayHandlerImpl {
        dispositions: dispositions.clone(),
    });
    Err(HfError::ServerFailed(err))
}

#[derive(Default)]
struct DispositionList {
    all_formats: FormatDisposition,
    formats: HashMap<u32, FormatDisposition>,
}

#[derive(Default)]
struct FormatDisposition {
    all_modifiers: Disposition,
    modifiers: HashMap<u64, Disposition>,
}

#[derive(Default)]
struct Disposition {
    allow: bool,
    deny_unless_allowed: bool,
}

fn map_shm_to_drm(v: u32) -> u32 {
    match v {
        0 => 0x34325241,
        1 => 0x34325258,
        _ => v,
    }
}

fn create_disposition_list(allow: &[Filter], deny: &[Filter]) -> Arc<DispositionList> {
    let mut dispositions = DispositionList::default();
    let mut add = |filters: &[Filter], allow: bool| {
        for filter in filters {
            let e = match filter.format {
                Some(mut v) => {
                    v = map_shm_to_drm(v);
                    dispositions.formats.entry(v).or_default()
                }
                _ => &mut dispositions.all_formats,
            };
            let d = match filter.modifier {
                Some(m) => e.modifiers.entry(m).or_default(),
                _ => &mut e.all_modifiers,
            };
            if allow {
                d.allow = true;
            } else {
                d.deny_unless_allowed = true;
            }
        }
    };
    add(allow, true);
    add(deny, false);
    if dispositions.all_formats.all_modifiers.allow {
        dispositions = DispositionList::default();
        dispositions.all_formats.all_modifiers.allow = true;
    }
    if dispositions.all_formats.all_modifiers.deny_unless_allowed {
        for format in dispositions.formats.values_mut() {
            format.all_modifiers.deny_unless_allowed = true;
        }
        dispositions
            .all_formats
            .modifiers
            .retain(|_, dis| dis.allow);
        dispositions.all_formats.modifiers.shrink_to_fit();
    }
    for format in dispositions.formats.values_mut() {
        if format.all_modifiers.allow {
            *format = FormatDisposition::default();
            format.all_modifiers.allow = true;
        }
        if format.all_modifiers.deny_unless_allowed {
            format.modifiers.retain(|_, dis| dis.allow);
            format.modifiers.shrink_to_fit();
        }
    }
    dispositions.formats.retain(|_, f| {
        f.all_modifiers.allow
            || (f.all_modifiers.deny_unless_allowed
                && !dispositions.all_formats.all_modifiers.deny_unless_allowed)
            || !f.modifiers.is_empty()
    });
    dispositions.formats.shrink_to_fit();
    Arc::new(dispositions)
}

impl DispositionList {
    fn allowed(&self, format: u32, modifier: u64) -> bool {
        if self.all_formats.all_modifiers.allow {
            return true;
        }
        let mut deny = false;
        if let Some(format) = self.formats.get(&format) {
            if format.all_modifiers.allow {
                return true;
            }
            if let Some(disposition) = format.modifiers.get(&modifier) {
                if disposition.allow {
                    return true;
                }
                deny |= disposition.deny_unless_allowed;
            }
            deny |= format.all_modifiers.deny_unless_allowed;
        }
        if let Some(disposition) = self.all_formats.modifiers.get(&modifier) {
            if disposition.allow {
                return true;
            }
            deny |= disposition.deny_unless_allowed;
        }
        deny |= self.all_formats.all_modifiers.deny_unless_allowed;
        if deny {
            return false;
        }
        true
    }
}

struct WlDisplayHandlerImpl {
    dispositions: Arc<DispositionList>,
}

impl WlDisplayHandler for WlDisplayHandlerImpl {
    fn handle_get_registry(&mut self, slf: &Rc<WlDisplay>, registry: &Rc<WlRegistry>) {
        registry.set_handler(WlRegistryHandlerImpl {
            dispositions: self.dispositions.clone(),
        });
        slf.send_get_registry(registry);
    }
}

struct WlRegistryHandlerImpl {
    dispositions: Arc<DispositionList>,
}

impl WlRegistryHandler for WlRegistryHandlerImpl {
    fn handle_bind(&mut self, slf: &Rc<WlRegistry>, name: u32, id: Rc<dyn Object>) {
        match id.interface() {
            WlShm::INTERFACE => id.downcast::<WlShm>().set_handler(WlShmHandlerImpl {
                dispositions: self.dispositions.clone(),
            }),
            WlDrm::INTERFACE => id.downcast::<WlDrm>().set_handler(WlDrmHandlerImpl {
                dispositions: self.dispositions.clone(),
            }),
            ZwpLinuxDmabufV1::INTERFACE => {
                id.downcast::<ZwpLinuxDmabufV1>()
                    .set_handler(ZwpLinuxDmabufV1HandlerImpl {
                        dispositions: self.dispositions.clone(),
                    })
            }
            _ => {}
        }
        slf.send_bind(name, id);
    }
}

pub const LINEAR_MODIFIER: u64 = 0;
pub const INVALID_MODIFIER: u64 = 0x00ff_ffff_ffff_ffff;

struct WlShmHandlerImpl {
    dispositions: Arc<DispositionList>,
}

impl WlShmHandler for WlShmHandlerImpl {
    fn handle_format(&mut self, slf: &Rc<WlShm>, format: WlShmFormat) {
        if self
            .dispositions
            .allowed(map_shm_to_drm(format.0), LINEAR_MODIFIER)
        {
            slf.send_format(format);
        }
    }
}

struct WlDrmHandlerImpl {
    dispositions: Arc<DispositionList>,
}

impl WlDrmHandler for WlDrmHandlerImpl {
    fn handle_format(&mut self, slf: &Rc<WlDrm>, format: u32) {
        if self.dispositions.allowed(format, INVALID_MODIFIER) {
            slf.send_format(format);
        }
    }
}

struct ZwpLinuxDmabufV1HandlerImpl {
    dispositions: Arc<DispositionList>,
}

impl ZwpLinuxDmabufV1Handler for ZwpLinuxDmabufV1HandlerImpl {
    fn handle_format(&mut self, slf: &Rc<ZwpLinuxDmabufV1>, format: u32) {
        if self.dispositions.allowed(format, INVALID_MODIFIER) {
            slf.send_format(format);
        }
    }

    fn handle_modifier(
        &mut self,
        slf: &Rc<ZwpLinuxDmabufV1>,
        format: u32,
        modifier_hi: u32,
        modifier_lo: u32,
    ) {
        let modifier = ((modifier_hi as u64) << 32) | modifier_lo as u64;
        if self.dispositions.allowed(format, modifier) {
            slf.send_format(format);
        }
    }

    fn handle_get_default_feedback(
        &mut self,
        slf: &Rc<ZwpLinuxDmabufV1>,
        id: &Rc<ZwpLinuxDmabufFeedbackV1>,
    ) {
        id.set_handler(ZwpLinuxDmabufFeedbackV1HandlerImpl {
            dispositions: self.dispositions.clone(),
            table: None,
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
            dispositions: self.dispositions.clone(),
            table: None,
        });
        slf.send_get_surface_feedback(id, surface);
    }
}

struct ZwpLinuxDmabufFeedbackV1HandlerImpl {
    dispositions: Arc<DispositionList>,
    table: Option<Mmap>,
}

impl ZwpLinuxDmabufFeedbackV1Handler for ZwpLinuxDmabufFeedbackV1HandlerImpl {
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

    fn handle_tranche_formats(&mut self, slf: &Rc<ZwpLinuxDmabufFeedbackV1>, indices: &[u8]) {
        let table = self.table.as_ref().unwrap();
        let mut out = vec![];
        for index in uapi::pod_iter::<u16, _>(indices).unwrap() {
            let format = uapi::pod_read_init::<u32, _>(&table[index as usize * 16..]).unwrap();
            let modifier =
                uapi::pod_read_init::<u64, _>(&table[index as usize * 16 + 8..]).unwrap();
            if self.dispositions.allowed(format, modifier) {
                out.push(index);
            }
        }
        slf.send_tranche_formats(uapi::as_bytes(&*out));
    }
}
