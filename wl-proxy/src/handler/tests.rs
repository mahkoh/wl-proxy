use {
    crate::handler::{HandlerHolder, HandlerMut, HandlerRef},
    std::{cell::Cell, rc::Rc},
};

#[test]
fn replace() {
    let holder = HandlerHolder::default();
    struct Handler(Rc<Cell<i32>>, i32);
    impl Drop for Handler {
        fn drop(&mut self) {
            self.0.set(self.1);
        }
    }
    let last_dropped = Rc::new(Cell::new(0));
    holder.set(Some(Box::new(Handler(last_dropped.clone(), 1))));
    assert_eq!(last_dropped.get(), 0);
    holder.set(None);
    assert_eq!(last_dropped.get(), 1);
    holder.set(Some(Box::new(Handler(last_dropped.clone(), 2))));
    assert_eq!(last_dropped.get(), 1);

    let borrow = holder.borrow_mut();
    holder.set(Some(Box::new(Handler(last_dropped.clone(), 3))));
    assert_eq!(last_dropped.get(), 1);
    drop(borrow);
    assert_eq!(last_dropped.get(), 2);

    let borrow = holder.borrow_mut();
    holder.set(Some(Box::new(Handler(last_dropped.clone(), 4))));
    assert_eq!(last_dropped.get(), 2);
    holder.set(Some(Box::new(Handler(last_dropped.clone(), 5))));
    assert_eq!(last_dropped.get(), 4);
    drop(borrow);
    assert_eq!(last_dropped.get(), 3);
}

#[test]
fn borrow() {
    let holder = HandlerHolder::default();
    holder.set(Some(Box::new(1)));
    assert_eq!(*holder.borrow_mut(), Some(Box::new(1)));
    assert_eq!(holder.try_borrow().as_deref(), Some(&Some(Box::new(1))));
    assert_eq!(holder.try_borrow_mut().as_deref(), Some(&Some(Box::new(1))));
    let mut borrow = holder.borrow_mut();
    assert_eq!(holder.try_borrow().as_deref(), None);
    assert_eq!(holder.try_borrow_mut().as_deref(), None);
    *borrow = Some(Box::new(2));
    drop(borrow);
    let _borrow = holder.try_borrow().unwrap();
    let borrow = holder.try_borrow().unwrap();
    assert_eq!(holder.try_borrow_mut().as_deref(), None);
    assert_eq!(*borrow, Some(Box::new(2)));
}

#[test]
#[should_panic]
fn multi_borrow() {
    let holder = HandlerHolder::default();
    holder.set(Some(Box::new(1)));
    let _borrow = holder.borrow_mut();
    holder.borrow_mut();
}

#[test]
fn map() {
    let holder = HandlerHolder::default();
    holder.set(Some(Box::new(1)));
    {
        let borrow = holder.borrow_mut();
        let borrow = HandlerMut::map(borrow, |b| b.as_deref_mut().unwrap());
        assert_eq!(*borrow, 1);
        assert_eq!(format!("{:?}", borrow), "1");
        assert_eq!(format!("{}", borrow), "1");
    }
    {
        let borrow = holder.try_borrow().unwrap();
        let borrow = HandlerRef::map(borrow, |b| b.as_deref().unwrap());
        assert_eq!(*borrow, 1);
        assert_eq!(format!("{:?}", borrow), "1");
        assert_eq!(format!("{}", borrow), "1");
    }
}

#[test]
fn clone() {
    let holder = HandlerHolder::default();
    holder.set(Some(Box::new(1)));
    let borrow = holder.try_borrow().unwrap();
    let borrow = HandlerRef::clone(&borrow);
    assert_eq!(*borrow, Some(Box::new(1)));
}
