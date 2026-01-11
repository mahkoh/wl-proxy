use {
    crate::{
        protocols::{
            ext_workspace_v1::ext_workspace_handle_v1::ExtWorkspaceHandleV1, wayland::wl_shm::WlShm,
        },
        test_framework::proxy::{test_proxy, test_proxy_no_log},
    },
    std::rc::Rc,
};

#[test]
fn test() {
    let tp = test_proxy();
    tp.sync();
}

#[test]
fn fd() {
    let tp = test_proxy();
    let wl_shm = tp.get_global::<WlShm>();
    let pool = Rc::new(uapi::memfd_create("", 0).unwrap().into());
    wl_shm.new_send_create_pool(&pool, 0);
    wl_shm.new_send_create_pool(&pool, 0);
    tp.sync();
}

#[test]
fn many_messages() {
    let tp = test_proxy_no_log();
    for _ in 0..100_000 {
        tp.client_display.new_send_sync();
    }
    tp.sync();
    tp.sync();
}

#[test]
fn many_messages_with_fd() {
    let tp = test_proxy_no_log();
    let wl_shm = tp.get_global::<WlShm>();
    let pool = Rc::new(uapi::memfd_create("", 0).unwrap().into());
    for _ in 0..1_000 {
        for _ in 0..100 {
            tp.client_display.new_send_sync();
        }
        wl_shm.new_send_create_pool(&pool, 0);
    }
    tp.sync();
    tp.sync();
}

#[test]
fn array() {
    let tp = test_proxy();
    let _ewh = tp.get_global::<ExtWorkspaceHandleV1>();
    tp.sync();
}
