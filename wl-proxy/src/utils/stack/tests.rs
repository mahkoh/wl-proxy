use crate::utils::stack::Stack;

#[test]
fn stack() {
    let stack = Stack::default();
    stack.push(0);
    stack.push(1);
    stack.push(2);
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.take(), vec![0, 1]);
}
