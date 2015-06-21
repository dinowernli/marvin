/// Represents a context tree.
pub struct ContextTree {
  root: Node,
}

impl ContextTree {
  pub fn create(depth: usize) -> ContextTree {
    ContextTree {
      root: Node::new(depth, 0),
    }
  }

  /// Returns the total number of nodes in the tree.
  pub fn size(&self) -> usize {
    self.root.size()
  }
}

/// One node in the context tree.
struct Node {
  left: Option<Box<Node>>,
  right: Option<Box<Node>>,
}

impl Node {
  fn new(depth: usize, level: usize) -> Node {
    return if level == depth {
      return Node::new_leaf();
    } else {
      return Node::new_inner(depth, level + 1);
    }
  }

  fn new_leaf() -> Node {
    Node {
      left: None,
      right: None,
    }
  }

  fn new_inner(depth: usize, level: usize) -> Node {
    Node {
      left: Some(Box::new(Node::new(depth, level + 1))),
      right: Some(Box::new(Node::new(depth, level + 1))),
    }
  }

  /// Returns the number of nodes in the subtree rooted at this node.
  fn size(&self) -> usize {
    return if self.is_leaf() { 1 } else {
      let left_size = self.left.as_ref().unwrap().size();
      let right_size = self.right.as_ref().unwrap().size();
      left_size + right_size + 1
    };
  }

  fn is_leaf(&self) -> bool {
    if self.left.is_some() != self.right.is_some() {
      panic!("ContextTree nodes cannot have only one child.");
    }
    return !self.left.is_some();
  }
}
