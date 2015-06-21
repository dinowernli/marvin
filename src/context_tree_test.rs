use context_tree::ContextTree;

#[test]
fn size() {
  let tree = ContextTree::create(4);
  assert_eq!(7, tree.size());  // 2^(n-1) - 1.
}

#[test]
fn empty() {
  let tree = ContextTree::create(0);
  assert_eq!(1, tree.size());
}
