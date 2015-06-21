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
  zero_child: Option<Box<Node>>,
  one_child: Option<Box<Node>>,
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
      zero_child: None,
      one_child: None,
    }
  }

  fn new_inner(depth: usize, level: usize) -> Node {
    Node {
      zero_child: Some(Box::new(Node::new(depth, level + 1))),
      one_child: Some(Box::new(Node::new(depth, level + 1))),
    }
  }

  /// Returns the number of nodes in the subtree rooted at this node.
  fn size(&self) -> usize {
    return if self.is_leaf() { 1 } else {
      let zero_child_size = self.zero_child.as_ref().unwrap().size();
      let one_child_size = self.one_child.as_ref().unwrap().size();
      zero_child_size + one_child_size + 1
    };
  }

  fn is_leaf(&self) -> bool {
    if self.zero_child.is_some() != self.one_child.is_some() {
      panic!("ContextTree nodes cannot have only one child.");
    }
    return !self.zero_child.is_some();
  }
}
