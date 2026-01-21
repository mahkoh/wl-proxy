use {
    quick_xml::{
        ElementWriter, Writer,
        events::{BytesDecl, BytesText, Event},
    },
    rusqlite::{Connection, Row, params},
    std::{
        cell::RefCell,
        collections::HashSet,
        fs::File,
        io::BufWriter,
        path::{Path, PathBuf},
    },
};

#[derive(Default)]
struct Repo {
    name: &'static str,
    dir: &'static str,
    allow: &'static [&'static str],
    block: &'static [&'static str],
}

fn main() {
    let db =
        reqwest::blocking::get("https://github.com/mahkoh/wayland-db/raw/refs/heads/db/wayland.db")
            .unwrap()
            .bytes()
            .unwrap();
    std::fs::write(".wayland.db", db).unwrap();
    let repos = [
        Repo {
            name: "hyprland-protocols",
            dir: "hyprland-protocols",
            allow: &[
                "hyprland_ctm_control_v1",
                "hyprland_focus_grab_v1",
                "hyprland_global_shortcuts_v1",
                "hyprland_input_capture_v1",
                "hyprland_lock_notify_v1",
                "hyprland_surface_v1",
                "hyprland_toplevel_export_v1",
                "hyprland_toplevel_mapping_v1",
            ],
            block: &[],
        },
        Repo {
            name: "jay-protocols",
            dir: "jay-protocols",
            allow: &["jay_popup_ext_v1", "jay_tray_v1"],
            block: &[],
        },
        Repo {
            name: "external",
            dir: "external",
            allow: &[
                "input_method_unstable_v2",
                "drm",
                "virtual_keyboard_unstable_v1",
            ],
            block: &[
                "agl_screenshooter",
                "agl_shell",
                "agl_shell_desktop",
                "aura_output_management",
                "aura_shell",
                "chrome_color_management",
                "gtk",
                "mir_shell_unstable_v1",
                "overlay_prioritizer",
                "stream",
                "surface_augmenter",
                "tizen_extension",
                "wl_eglstream_controller",
            ],
        },
        // excluded: they make breaking changes to their protocols
        // Repo {
        //     name: "treeland-protocols",
        //     dir: "treeland-protocols",
        //     allow: &[
        //         "treeland_app_id_resolver_v1",
        //         "treeland_dde_shell_v1",
        //         "treeland_ddm",
        //         "treeland_foreign_toplevel_manager_v1",
        //         "treeland_output_manager_v1",
        //         "treeland_personalization_manager_v1",
        //         "treeland_prelaunch_splash_v1",
        //         "treeland_screensaver",
        //         "treeland_shortcut_manager_v1",
        //         "treeland_shortcut_manager_v2",
        //         "treeland_virtual_output_manager_v1",
        //         "treeland_wallpaper_color_v1",
        //         "treeland_window_management_v1",
        //     ],
        //     block: &["treeland_capture_unstable_v1"],
        // },
        Repo {
            name: "wayland",
            dir: "wayland",
            allow: &["wayland"],
            block: &[],
        },
        Repo {
            name: "wayland-protocols",
            dir: "wayland-protocols",
            allow: &[
                "alpha_modifier_v1",
                "color_management_v1",
                "color_representation_v1",
                "commit_timing_v1",
                "content_type_v1",
                "cursor_shape_v1",
                "drm_lease_v1",
                "ext_background_effect_v1",
                "ext_data_control_v1",
                "ext_foreign_toplevel_list_v1",
                "ext_idle_notify_v1",
                "ext_image_capture_source_v1",
                "ext_image_copy_capture_v1",
                "ext_session_lock_v1",
                "ext_transient_seat_v1",
                "ext_workspace_v1",
                "fifo_v1",
                "fractional_scale_v1",
                "fullscreen_shell_unstable_v1",
                "idle_inhibit_unstable_v1",
                "input_method_unstable_v1",
                "input_timestamps_unstable_v1",
                "keyboard_shortcuts_inhibit_unstable_v1",
                "linux_dmabuf_v1",
                "linux_drm_syncobj_v1",
                "pointer_constraints_unstable_v1",
                "pointer_gestures_unstable_v1",
                "pointer_warp_v1",
                "presentation_time",
                "relative_pointer_unstable_v1",
                "security_context_v1",
                "single_pixel_buffer_v1",
                "tablet_v2",
                "tearing_control_v1",
                "text_input_unstable_v1",
                "text_input_unstable_v3",
                "viewporter",
                "wp_primary_selection_unstable_v1",
                "xdg_activation_v1",
                "xdg_decoration_unstable_v1",
                "xdg_dialog_v1",
                "xdg_foreign_unstable_v2",
                "xdg_output_unstable_v1",
                "xdg_shell",
                "xdg_system_bell_v1",
                "xdg_toplevel_drag_v1",
                "xdg_toplevel_icon_v1",
                "xdg_toplevel_tag_v1",
                "xwayland_keyboard_grab_unstable_v1",
                "xwayland_shell_v1",
                "zwp_linux_explicit_synchronization_unstable_v1",
            ],
            block: &[
                "linux_dmabuf_unstable_v1",
                "tablet_unstable_v1",
                "tablet_unstable_v2",
                "xdg_foreign_unstable_v1",
                "xdg_shell_unstable_v5",
                "xdg_shell_unstable_v6",
                "input_method_experimental_v2",
                "keyboard_filter_experimental_v1",
                "xx_session_management_v1",
                "xx_text_input_unstable_v3",
            ],
        },
        Repo {
            name: "wlr-protocols",
            dir: "wlr-protocols",
            allow: &[
                "wlr_data_control_unstable_v1",
                "wlr_export_dmabuf_unstable_v1",
                "wlr_foreign_toplevel_management_unstable_v1",
                "wlr_gamma_control_unstable_v1",
                "wlr_input_inhibit_unstable_v1",
                "wlr_layer_shell_unstable_v1",
                "wlr_output_management_unstable_v1",
                "wlr_output_power_management_unstable_v1",
                "wlr_screencopy_unstable_v1",
                "wlr_virtual_pointer_unstable_v1",
            ],
            block: &[],
        },
        Repo {
            name: "river",
            dir: "river-protocols",
            allow: &[
                "river_input_management_v1",
                "river_layer_shell_v1",
                "river_libinput_config_v1",
                "river_window_management_v1",
                "river_xkb_bindings_v1",
                "river_xkb_config_v1",
            ],
            block: &[],
        },
        Repo {
            name: "weston",
            dir: "weston-protocols",
            allow: &[
                "ivi_application",
                "ivi_hmi_controller",
                "weston_content_protection",
                "weston_debug",
                "weston_desktop",
                "weston_direct_display",
                "weston_output_capture",
                "weston_test",
                "weston_touch_calibration",
            ],
            block: &["text_cursor_position"],
        },
        Repo {
            name: "cosmic-protocols",
            dir: "cosmic-protocols",
            allow: &[
                "cosmic_a11y_v1",
                "cosmic_corner_radius_v1",
                "cosmic_image_source_unstable_v1",
                "cosmic_output_management_unstable_v1",
                "cosmic_overlap_notify_unstable_v1",
                "cosmic_workspace_unstable_v2",
            ],
            block: &[
                "cosmic_workspace_unstable_v1",
                "cosmic_toplevel_info_unstable_v1",
                "cosmic_toplevel_management_unstable_v1",
            ],
        },
    ];
    let db = Connection::open(".wayland.db").unwrap();
    let mut query_protocols = db
        // language=sqlite
        .prepare("select p.* from repo r join protocol p using (repo_id) where r.name = $1")
        .unwrap();
    let mut query_interfaces = db
        // language=sqlite
        .prepare("select * from interface where protocol_id = $1 order by interface_id")
        .unwrap();
    let mut query_messages = db
        // language=sqlite
        .prepare("select * from message where interface_id = $1 order by message_id")
        .unwrap();
    let mut query_args = db
        // language=sqlite
        .prepare("select a.*, t.name type_name from arg a join type t using (type_id) where a.message_id = $1 order by position")
        .unwrap();
    let mut query_enums = db
        // language=sqlite
        .prepare("select * from enum where interface_id = $1 order by enum_id")
        .unwrap();
    let mut query_entries = db
        // language=sqlite
        .prepare("select * from entry where enum_id = $1 order by entry_id")
        .unwrap();
    let query_description = db
        // language=sqlite
        .prepare("select * from description where description_id = $1")
        .unwrap();
    let query_description = RefCell::new(query_description);
    let write_text = |w: ElementWriter<BufWriter<File>>, text: &str| {
        if text.is_empty() {
            w.write_empty().unwrap();
        } else {
            w.write_inner_content(|w| {
                for line in text.lines() {
                    if line.is_empty() {
                        w.write_event(Event::Text(BytesText::new("\n")))?;
                    } else {
                        w.write_indent()?;
                        w.write_event(Event::Text(BytesText::from_escaped(line)))?;
                    }
                }
                w.write_event(Event::Eof)?;
                Ok(())
            })
            .unwrap();
        }
    };
    let write_description = |w: &mut Writer<BufWriter<File>>, description_id: Option<i64>| {
        let Some(description_id) = description_id else {
            return;
        };
        let query_description = &mut *query_description.borrow_mut();
        let mut rows = query_description.query(params![description_id]).unwrap();
        let row = rows.next().unwrap().unwrap();
        let summary: Option<String> = row.get("summary").unwrap();
        let body: String = row.get("body").unwrap();
        let mut element = w.create_element("description");
        if let Some(summary) = summary {
            element = element.with_attribute(("summary", &*summary));
        }
        write_text(element, &body);
    };
    type W = Writer<BufWriter<File>>;
    let write_entry = |w: &mut W, row: &Row| {
        let name: String = row.get("name").unwrap();
        let value_str: String = row.get("value_str").unwrap();
        let summary: Option<String> = row.get("summary").unwrap();
        let since: Option<i64> = row.get("since").unwrap();
        let deprecated_since: Option<i64> = row.get("deprecated_since").unwrap();
        let description_id: Option<i64> = row.get("description_id").unwrap();
        let mut e = w
            .create_element("entry")
            .with_attribute(("name", &*name))
            .with_attribute(("value", &*value_str));
        if let Some(since) = since {
            let since = since.to_string();
            e = e.with_attribute(("since", &*since));
        }
        if let Some(since) = deprecated_since {
            let since = since.to_string();
            e = e.with_attribute(("deprecated-since", &*since));
        }
        if let Some(summary) = &summary {
            e = e.with_attribute(("summary", &**summary));
        }
        if description_id.is_some() {
            e.write_inner_content(|w| {
                write_description(w, description_id);
                Ok(())
            })
            .unwrap();
        } else {
            e.write_empty().unwrap();
        }
    };
    let mut write_enum = |w: &mut W, row: &Row| {
        let enum_id: i64 = row.get("enum_id").unwrap();
        let name: String = row.get("name").unwrap();
        let since: Option<i64> = row.get("since").unwrap();
        let is_bitfield: bool = row.get("is_bitfield").unwrap();
        let description_id: Option<i64> = row.get("description_id").unwrap();
        let mut e = w.create_element("enum").with_attribute(("name", &*name));
        if let Some(since) = since {
            let since = since.to_string();
            e = e.with_attribute(("since", &*since));
        }
        if is_bitfield {
            e = e.with_attribute(("bitfield", "true"));
        }
        e.write_inner_content(|w| {
            write_description(w, description_id);
            let mut entries = query_entries.query(params![enum_id]).unwrap();
            while let Some(row) = entries.next().unwrap() {
                write_entry(w, row);
            }
            Ok(())
        })
        .unwrap();
    };
    let write_arg = |w: &mut W, row: &Row| {
        let name: String = row.get("name").unwrap();
        let type_name: String = row.get("type_name").unwrap();
        let summary: Option<String> = row.get("summary").unwrap();
        let description_id: Option<i64> = row.get("description_id").unwrap();
        let interface_name: Option<String> = row.get("interface_name").unwrap();
        let allow_null: bool = row.get("allow_null").unwrap();
        let enum_name: Option<String> = row.get("enum_name").unwrap();
        let mut e = w
            .create_element("arg")
            .with_attribute(("name", &*name))
            .with_attribute(("type", &*type_name));
        if let Some(name) = &interface_name {
            e = e.with_attribute(("interface", &**name));
        }
        if let Some(name) = &enum_name {
            e = e.with_attribute(("enum", &**name));
        }
        if allow_null {
            e = e.with_attribute(("allow-null", "true"));
        }
        if let Some(summary) = &summary {
            e = e.with_attribute(("summary", &**summary));
        }
        if description_id.is_some() {
            e.write_inner_content(|w| {
                write_description(w, description_id);
                Ok(())
            })
            .unwrap();
        } else {
            e.write_empty().unwrap();
        }
    };
    let mut write_message = |w: &mut W, row: &Row| {
        let message_id: i64 = row.get("message_id").unwrap();
        let name: String = row.get("name").unwrap();
        let is_request: bool = row.get("is_request").unwrap();
        let is_destructor: bool = row.get("is_destructor").unwrap();
        let since: Option<i64> = row.get("since").unwrap();
        let deprecated_since: Option<i64> = row.get("deprecated_since").unwrap();
        let description_id: Option<i64> = row.get("description_id").unwrap();
        let element_name = match is_request {
            true => "request",
            false => "event",
        };
        let mut e = w
            .create_element(element_name)
            .with_attribute(("name", &*name));
        if is_destructor {
            e = e.with_attribute(("type", "destructor"));
        }
        if let Some(since) = since {
            let since = since.to_string();
            e = e.with_attribute(("since", &*since));
        }
        if let Some(since) = deprecated_since {
            let since = since.to_string();
            e = e.with_attribute(("deprecated-since", &*since));
        }
        e.write_inner_content(|w| {
            write_description(w, description_id);
            let mut args = query_args.query(params![message_id]).unwrap();
            while let Some(row) = args.next().unwrap() {
                write_arg(w, row);
            }
            Ok(())
        })
        .unwrap();
    };
    let mut write_interface = |w: &mut W, row: &Row| {
        let interface_id: i64 = row.get("interface_id").unwrap();
        let name: String = row.get("name").unwrap();
        let version: i64 = row.get("version").unwrap();
        let version = version.to_string();
        let description_id: Option<i64> = row.get("description_id").unwrap();
        w.create_element("interface")
            .with_attribute(("name", &*name))
            .with_attribute(("version", &*version))
            .write_inner_content(|w| {
                write_description(w, description_id);
                let mut messages = query_messages.query(params![interface_id]).unwrap();
                while let Some(row) = messages.next().unwrap() {
                    write_message(w, row);
                }
                let mut enums = query_enums.query(params![interface_id]).unwrap();
                while let Some(row) = enums.next().unwrap() {
                    write_enum(w, row);
                }
                Ok(())
            })
            .unwrap();
    };
    for repo in repos {
        let allow: HashSet<_> = repo.allow.iter().copied().collect();
        let block: HashSet<_> = repo.block.iter().copied().collect();
        let mut protocols = query_protocols.query(params![repo.name]).unwrap();
        while let Some(row) = protocols.next().unwrap() {
            let protocol_id: i64 = row.get("protocol_id").unwrap();
            let name: String = row.get("name").unwrap();
            let path: String = row.get("path").unwrap();
            let copyright: Option<String> = row.get("copyright").unwrap();
            let description_id: Option<i64> = row.get("description_id").unwrap();
            if block.contains(&*name) {
                continue;
            }
            if !allow.contains(&*name) {
                panic!(
                    "protocol {}.{name} is neither blocked nor allowed",
                    repo.name
                );
            }
            let file_name = Path::new(&path).file_name().unwrap().to_str().unwrap();
            let mut path = PathBuf::new();
            path.push("protocols");
            path.push(repo.dir);
            std::fs::create_dir_all(&path).unwrap();
            path.push(file_name);
            let file = BufWriter::new(
                File::options()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&path)
                    .unwrap(),
            );
            let mut writer = quick_xml::Writer::new_with_indent(file, b' ', 2);
            writer
                .write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))
                .unwrap();
            writer
                .create_element("protocol")
                .with_attribute(("name", &*name))
                .write_inner_content(|w| {
                    if let Some(copyright) = copyright {
                        write_text(w.create_element("copyright"), &copyright);
                    }
                    write_description(w, description_id);
                    let mut interfaces = query_interfaces.query(params![protocol_id]).unwrap();
                    while let Some(row) = interfaces.next().unwrap() {
                        write_interface(w, row);
                    }
                    Ok(())
                })
                .unwrap();
        }
    }
}
