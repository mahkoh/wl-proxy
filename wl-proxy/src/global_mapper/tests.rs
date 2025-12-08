use {
    crate::{
        global_mapper::{GlobalMapper, RegistryApi},
        object::{Object, ObjectCoreApi, ObjectError, ObjectErrorKind},
        protocols::ObjectInterface,
        test_framework::proxy::test_proxy_no_log,
    },
    std::{cell::RefCell, collections::VecDeque, rc::Rc},
};

#[derive(Debug)]
struct ObjEqWrapper(Rc<dyn Object>);

impl PartialEq for ObjEqWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.unique_id() == other.0.unique_id()
    }
}

impl Eq for ObjEqWrapper {}

#[derive(Eq, PartialEq, Debug)]
enum RegistryMsg {
    Bind(u32, ObjEqWrapper),
    Global(u32, ObjectInterface, u32),
    GlobalRemove(u32),
}

impl RegistryApi for RefCell<&'_ mut VecDeque<RegistryMsg>> {
    fn bind(&self, name: u32, id: Rc<dyn Object>) -> Result<(), ObjectError> {
        self.borrow_mut()
            .push_back(RegistryMsg::Bind(name, ObjEqWrapper(id)));
        Ok(())
    }

    fn global(
        &self,
        name: u32,
        interface: ObjectInterface,
        version: u32,
    ) -> Result<(), ObjectError> {
        self.borrow_mut()
            .push_back(RegistryMsg::Global(name, interface, version));
        Ok(())
    }

    fn global_remove(&self, name: u32) -> Result<(), ObjectError> {
        self.borrow_mut().push_back(RegistryMsg::GlobalRemove(name));
        Ok(())
    }
}

struct ErrorRegistry {
    fail_bind: bool,
    fail_global: bool,
    fail_global_remove: bool,
}

impl ErrorRegistry {
    fn new() -> Self {
        Self {
            fail_bind: false,
            fail_global: false,
            fail_global_remove: false,
        }
    }
}

impl RegistryApi for RefCell<ErrorRegistry> {
    fn bind(&self, _name: u32, _id: Rc<dyn Object>) -> Result<(), ObjectError> {
        if self.borrow().fail_bind {
            Err(ObjectErrorKind::HandlerBorrowed.into())
        } else {
            Ok(())
        }
    }

    fn global(
        &self,
        _name: u32,
        _interface: ObjectInterface,
        _version: u32,
    ) -> Result<(), ObjectError> {
        if self.borrow().fail_global {
            Err(ObjectErrorKind::HandlerBorrowed.into())
        } else {
            Ok(())
        }
    }

    fn global_remove(&self, _name: u32) -> Result<(), ObjectError> {
        if self.borrow().fail_global_remove {
            Err(ObjectErrorKind::HandlerBorrowed.into())
        } else {
            Ok(())
        }
    }
}

#[test]
fn test() {
    let mut events = VecDeque::new();
    macro_rules! events {
        () => {
            &mut RefCell::new(&mut events)
        };
    }
    let mut mapper = GlobalMapper::default();
    let kb_name = mapper.add_synthetic_global_impl(events!(), ObjectInterface::WlKeyboard, 1);
    assert_eq!(kb_name, 1);
    assert_eq!(
        events.pop_front(),
        Some(RegistryMsg::Global(1, ObjectInterface::WlKeyboard, 1))
    );
    assert_eq!(events.pop_front(), None);
    let pointer_name = mapper.add_synthetic_global_impl(events!(), ObjectInterface::WlPointer, 2);
    assert_eq!(pointer_name, 2);
    assert_eq!(
        events.pop_front(),
        Some(RegistryMsg::Global(2, ObjectInterface::WlPointer, 2))
    );
    assert_eq!(events.pop_front(), None);
    mapper.forward_global_impl(events!(), 1, ObjectInterface::WlShm, 4);
    assert_eq!(
        events.pop_front(),
        Some(RegistryMsg::Global(3, ObjectInterface::WlShm, 4))
    );
    assert_eq!(events.pop_front(), None);
}

#[test]
fn test_default() {
    let mapper = GlobalMapper::default();
    assert_eq!(mapper.server_to_client.get(&0), Some(&None));
    assert_eq!(mapper.client_to_server.len(), 1);
    assert_eq!(mapper.client_to_server[0], None);
}

#[test]
fn test_try_add_synthetic_global() {
    let mut events = VecDeque::new();
    let registry = RefCell::new(&mut events);
    let mut mapper = GlobalMapper::default();

    let name = mapper
        .try_add_synthetic_global_impl(&registry, ObjectInterface::WlShm, 1)
        .unwrap();
    assert_eq!(name, 1);
    assert_eq!(
        events.pop_front(),
        Some(RegistryMsg::Global(1, ObjectInterface::WlShm, 1))
    );
}

#[test]
fn test_try_add_synthetic_global_error() {
    let mut registry = ErrorRegistry::new();
    registry.fail_global = true;
    let registry = RefCell::new(registry);
    let mut mapper = GlobalMapper::default();

    let result = mapper.try_add_synthetic_global_impl(&registry, ObjectInterface::WlShm, 1);
    assert!(result.is_err());
}

#[test]
fn test_add_synthetic_global_error() {
    let mut registry = ErrorRegistry::new();
    registry.fail_global = true;
    let registry = RefCell::new(registry);
    let mut mapper = GlobalMapper::default();

    // Should return name even on error
    let name = mapper.add_synthetic_global_impl(&registry, ObjectInterface::WlShm, 1);
    assert_eq!(name, 1);
}

#[test]
fn test_remove_synthetic_global() {
    let mut events = VecDeque::new();
    let mut mapper = GlobalMapper::default();

    let name =
        mapper.add_synthetic_global_impl(&RefCell::new(&mut events), ObjectInterface::WlShm, 1);
    events.clear();

    mapper.remove_synthetic_global_impl(&RefCell::new(&mut events), name);
    assert_eq!(events.pop_front(), Some(RegistryMsg::GlobalRemove(1)));
}

#[test]
fn test_try_remove_synthetic_global() {
    let mut events = VecDeque::new();
    let mut mapper = GlobalMapper::default();

    let name =
        mapper.add_synthetic_global_impl(&RefCell::new(&mut events), ObjectInterface::WlShm, 1);
    events.clear();

    mapper
        .try_remove_synthetic_global_impl(&RefCell::new(&mut events), name)
        .unwrap();
    assert_eq!(events.pop_front(), Some(RegistryMsg::GlobalRemove(1)));
}

#[test]
fn test_try_remove_synthetic_global_error() {
    let mut registry = ErrorRegistry::new();
    registry.fail_global_remove = true;
    let registry = RefCell::new(registry);
    let mut mapper = GlobalMapper::default();

    let result = mapper.try_remove_synthetic_global_impl(&registry, 1);
    assert!(result.is_err());
}

#[test]
fn test_remove_synthetic_global_error() {
    let mut registry = ErrorRegistry::new();
    registry.fail_global_remove = true;
    let registry = RefCell::new(registry);
    let mut mapper = GlobalMapper::default();

    // Should not panic on error
    mapper.remove_synthetic_global_impl(&registry, 1);
}

#[test]
fn test_forward_global() {
    let mut events = VecDeque::new();
    let registry = RefCell::new(&mut events);
    let mut mapper = GlobalMapper::default();

    mapper.forward_global_impl(&registry, 100, ObjectInterface::WlCompositor, 5);
    assert_eq!(
        events.pop_front(),
        Some(RegistryMsg::Global(1, ObjectInterface::WlCompositor, 5))
    );

    // Check mapping
    assert_eq!(mapper.server_to_client.get(&100), Some(&Some(1)));
    assert_eq!(mapper.client_to_server.get(1), Some(&Some(100)));
}

#[test]
fn test_try_forward_global() {
    let mut events = VecDeque::new();
    let registry = RefCell::new(&mut events);
    let mut mapper = GlobalMapper::default();

    mapper
        .try_forward_global_impl(&registry, 100, ObjectInterface::WlCompositor, 5)
        .unwrap();
    assert_eq!(
        events.pop_front(),
        Some(RegistryMsg::Global(1, ObjectInterface::WlCompositor, 5))
    );
}

#[test]
fn test_try_forward_global_error() {
    let mut registry = ErrorRegistry::new();
    registry.fail_global = true;
    let registry = RefCell::new(registry);
    let mut mapper = GlobalMapper::default();

    let result = mapper.try_forward_global_impl(&registry, 100, ObjectInterface::WlCompositor, 5);
    assert!(result.is_err());
}

#[test]
fn test_forward_global_error() {
    let mut registry = ErrorRegistry::new();
    registry.fail_global = true;
    let registry = RefCell::new(registry);
    let mut mapper = GlobalMapper::default();

    // Should not panic on error
    mapper.forward_global_impl(&registry, 100, ObjectInterface::WlCompositor, 5);
}

#[test]
fn test_ignore_global() {
    let mut mapper = GlobalMapper::default();

    mapper.ignore_global(50);
    assert_eq!(mapper.server_to_client.get(&50), Some(&None));
}

#[test]
fn test_forward_global_remove() {
    let mut events = VecDeque::new();
    let mut mapper = GlobalMapper::default();

    // Add a global first
    mapper.forward_global_impl(
        &RefCell::new(&mut events),
        100,
        ObjectInterface::WlCompositor,
        5,
    );
    events.clear();

    // Remove it
    mapper.forward_global_remove_impl(&RefCell::new(&mut events), 100);
    assert_eq!(events.pop_front(), Some(RegistryMsg::GlobalRemove(1)));

    // Check it's removed from mapping
    assert_eq!(mapper.server_to_client.get(&100), None);
}

#[test]
fn test_try_forward_global_remove() {
    let mut events = VecDeque::new();
    let mut mapper = GlobalMapper::default();

    // Add a global first
    mapper.forward_global_impl(
        &RefCell::new(&mut events),
        100,
        ObjectInterface::WlCompositor,
        5,
    );
    events.clear();

    // Remove it
    mapper
        .try_forward_global_remove_impl(&RefCell::new(&mut events), 100)
        .unwrap();
    assert_eq!(events.pop_front(), Some(RegistryMsg::GlobalRemove(1)));
}

#[test]
fn test_forward_global_remove_nonexistent() {
    let mut events = VecDeque::new();
    let registry = RefCell::new(&mut events);
    let mut mapper = GlobalMapper::default();

    // Try to remove a global that doesn't exist - should not panic
    mapper.forward_global_remove_impl(&registry, 999);
    assert_eq!(events.pop_front(), None);
}

#[test]
fn test_try_forward_global_remove_nonexistent() {
    let mut events = VecDeque::new();
    let registry = RefCell::new(&mut events);
    let mut mapper = GlobalMapper::default();

    // Try to remove a global that doesn't exist - should succeed
    mapper
        .try_forward_global_remove_impl(&registry, 999)
        .unwrap();
    assert_eq!(events.pop_front(), None);
}

#[test]
fn test_forward_global_remove_ignored() {
    let mut events = VecDeque::new();
    let registry = RefCell::new(&mut events);
    let mut mapper = GlobalMapper::default();

    // Add an ignored global
    mapper.ignore_global(50);

    // Remove it - should not send event to client
    mapper.forward_global_remove_impl(&registry, 50);
    assert_eq!(events.pop_front(), None);
}

#[test]
fn test_try_forward_global_remove_ignored() {
    let mut events = VecDeque::new();
    let registry = RefCell::new(&mut events);
    let mut mapper = GlobalMapper::default();

    // Add an ignored global
    mapper.ignore_global(50);

    // Remove it - should not send event to client
    mapper
        .try_forward_global_remove_impl(&registry, 50)
        .unwrap();
    assert_eq!(events.pop_front(), None);
}

#[test]
fn test_try_forward_global_remove_error() {
    let mut events = VecDeque::new();
    let events_ref = RefCell::new(&mut events);
    let mut mapper = GlobalMapper::default();

    // Add a global first
    mapper.forward_global_impl(&events_ref, 100, ObjectInterface::WlCompositor, 5);

    // Now try to remove with error registry
    let mut registry = ErrorRegistry::new();
    registry.fail_global_remove = true;
    let registry = RefCell::new(registry);

    let result = mapper.try_forward_global_remove_impl(&registry, 100);
    assert!(result.is_err());
}

#[test]
fn test_forward_global_remove_error() {
    let mut events = VecDeque::new();
    let events_ref = RefCell::new(&mut events);
    let mut mapper = GlobalMapper::default();

    // Add a global first
    mapper.forward_global_impl(&events_ref, 100, ObjectInterface::WlCompositor, 5);

    // Now try to remove with error registry
    let mut registry = ErrorRegistry::new();
    registry.fail_global_remove = true;
    let registry = RefCell::new(registry);

    // Should not panic on error
    mapper.forward_global_remove_impl(&registry, 100);
}

#[test]
fn test_forward_bind() {
    let proxy = test_proxy_no_log();
    let mut events = VecDeque::new();
    let mut mapper = GlobalMapper::default();

    // Add a global first
    mapper.forward_global_impl(
        &RefCell::new(&mut events),
        100,
        ObjectInterface::WlCompositor,
        5,
    );
    events.clear();

    // Bind to it
    let obj = proxy.client_display.clone() as Rc<dyn Object>;
    mapper.forward_bind_impl(&RefCell::new(&mut events), 1, &obj);
    assert_eq!(
        events.pop_front(),
        Some(RegistryMsg::Bind(100, ObjEqWrapper(obj)))
    );
}

#[test]
fn test_try_forward_bind() {
    let proxy = test_proxy_no_log();
    let mut events = VecDeque::new();
    let mut mapper = GlobalMapper::default();

    // Add a global first
    mapper.forward_global_impl(
        &RefCell::new(&mut events),
        100,
        ObjectInterface::WlCompositor,
        5,
    );
    events.clear();

    // Bind to it
    let obj = proxy.client_display.clone() as Rc<dyn Object>;
    mapper
        .try_forward_bind_impl(&RefCell::new(&mut events), 1, &obj)
        .unwrap();
    assert_eq!(
        events.pop_front(),
        Some(RegistryMsg::Bind(100, ObjEqWrapper(obj)))
    );
}

#[test]
fn test_forward_bind_nonexistent() {
    let proxy = test_proxy_no_log();
    let mut events = VecDeque::new();
    let registry = RefCell::new(&mut events);
    let mut mapper = GlobalMapper::default();

    // Try to bind to a global that doesn't exist - should not panic
    let obj = proxy.client_display.clone() as Rc<dyn Object>;
    mapper.forward_bind_impl(&registry, 999, &obj);
    assert_eq!(events.pop_front(), None);
}

#[test]
fn test_try_forward_bind_nonexistent() {
    let proxy = test_proxy_no_log();
    let mut events = VecDeque::new();
    let registry = RefCell::new(&mut events);
    let mut mapper = GlobalMapper::default();

    // Try to bind to a global that doesn't exist - should succeed
    let obj = proxy.client_display.clone() as Rc<dyn Object>;
    mapper.try_forward_bind_impl(&registry, 999, &obj).unwrap();
    assert_eq!(events.pop_front(), None);
}

#[test]
fn test_forward_bind_synthetic() {
    let proxy = test_proxy_no_log();
    let mut events = VecDeque::new();
    let mut mapper = GlobalMapper::default();

    // Add a synthetic global
    let name =
        mapper.add_synthetic_global_impl(&RefCell::new(&mut events), ObjectInterface::WlShm, 1);
    events.clear();

    // Try to bind to it - should not forward to server
    let obj = proxy.client_display.clone() as Rc<dyn Object>;
    mapper.forward_bind_impl(&RefCell::new(&mut events), name, &obj);
    assert_eq!(events.pop_front(), None);
}

#[test]
fn test_try_forward_bind_synthetic() {
    let proxy = test_proxy_no_log();
    let mut events = VecDeque::new();
    let mut mapper = GlobalMapper::default();

    // Add a synthetic global
    let name =
        mapper.add_synthetic_global_impl(&RefCell::new(&mut events), ObjectInterface::WlShm, 1);
    events.clear();

    // Try to bind to it - should not forward to server
    let obj = proxy.client_display.clone() as Rc<dyn Object>;
    mapper
        .try_forward_bind_impl(&RefCell::new(&mut events), name, &obj)
        .unwrap();
    assert_eq!(events.pop_front(), None);
}

#[test]
fn test_try_forward_bind_error() {
    let proxy = test_proxy_no_log();
    let mut events = VecDeque::new();
    let events_ref = RefCell::new(&mut events);
    let mut mapper = GlobalMapper::default();

    // Add a global first
    mapper.forward_global_impl(&events_ref, 100, ObjectInterface::WlCompositor, 5);

    // Now try to bind with error registry
    let mut registry = ErrorRegistry::new();
    registry.fail_bind = true;
    let registry = RefCell::new(registry);

    let obj = proxy.client_display.clone() as Rc<dyn Object>;
    let result = mapper.try_forward_bind_impl(&registry, 1, &obj);
    assert!(result.is_err());
}

#[test]
fn test_forward_bind_error() {
    let proxy = test_proxy_no_log();
    let mut events = VecDeque::new();
    let events_ref = RefCell::new(&mut events);
    let mut mapper = GlobalMapper::default();

    // Add a global first
    mapper.forward_global_impl(&events_ref, 100, ObjectInterface::WlCompositor, 5);

    // Now try to bind with error registry
    let mut registry = ErrorRegistry::new();
    registry.fail_bind = true;
    let registry = RefCell::new(registry);

    // Should not panic on error
    let obj = proxy.client_display.clone() as Rc<dyn Object>;
    mapper.forward_bind_impl(&registry, 1, &obj);
}
