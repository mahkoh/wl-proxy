use crate::utils::stash::Stash;

#[test]
fn test() {
    let stash = Stash::default();
    stash.borrow().push(0);
    assert_eq!(stash.borrow().len(), 0);
}
