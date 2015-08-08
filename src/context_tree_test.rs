use context_tree::ContextTree;
use context_tree::Predictor;

#[test]
fn size() {
  let tree = ContextTree::create(3);
  assert_eq!(15, tree.size());  // 2^(n+1) - 1.
}

#[test]
fn empty() {
  let tree = ContextTree::create(0);
  assert_eq!(1, tree.size());
}

#[test]
#[should_panic]
fn invalid_revert() {
  let mut tree = ContextTree::create(0);
  assert_eq!(0, tree.history_size());

  // Attempt to revert to something greater than the history size.
  tree.revert_to_history_size(17);
}
