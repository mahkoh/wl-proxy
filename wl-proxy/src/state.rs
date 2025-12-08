use crate::free_list::FreeList;
use crate::proxy::Proxy;
use crate::trans::OutputSwapchain;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct InnerState {
    pub outgoing: RefCell<OutputSwapchain>,
    pub objects: RefCell<HashMap<u32, Rc<dyn Proxy>>>,
    pub idl: FreeList<u32, 3>,
}

impl InnerState {
    pub fn lookup(&self, id: u32) -> Option<Rc<dyn Proxy>> {
        self.objects.borrow().get(&id).cloned()
    }
}
