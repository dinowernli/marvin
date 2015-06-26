use bitstring::Bit;
use bitstring::Bitstring;

// Open questions:
// - How to declare some fields final
// - How to declare some fields mutable (can be changed for non-mut self)
// - Have the tree implement iter and simplify some methods with "fold".

/// An object capable of predicting observations and rewards based on
/// experience.
pub trait Predictor {
}

/// Context tree which computes a probability estimate for a sequence of
/// bits based on a mixture model of Markov chains. Lower-degree Markov
/// chains have higher priors in the mixture model, thus formalizing the
/// notion of Occam's Razor.
///
/// In order to get a probability for a sequence, all bits in the sequence
/// must be processed in-order.
pub struct ContextTree {
  root: Node,
  history: Bitstring,
  depth: usize,
}

impl Predictor for ContextTree {
}

impl ContextTree {
  /// The depth of the tree is the distance between leaves and the root.
  /// For a given depth d, the tree will compute a mixture model of all
  /// Markov chains of degree at most d.
  pub fn create(depth: usize) -> Self {
    ContextTree {
      root: Node::create_root(depth, 0),
      history: Bitstring::new(),
      depth: depth,
    }
  }

  /// Returns the total number of nodes in the tree.
  pub fn size(&self) -> usize {
    self.root.size()
  }

  pub fn history_size(&self) -> usize {
    self.history.len()
  }

  pub fn update(&mut self, bitstring: &Bitstring) {
    for bit in bitstring.bits() {
      self.update_bit(*bit);
    }
  }

  fn update_bit(&mut self, bit: Bit) {
    if self.history_size() < self.depth {
      self.history.push(bit);
      return;
    }
    // TODO(dinowernli): Find current context path and update from leaf
    // to root.
  }

  /// Returns log2 of the estimated probability of the current history.
  fn log_block_prob(&mut self) -> f64 {
    return self.root.log_weighted_prob();
  }
}

/// One node in the context tree. Maintains a count of the number
/// of zeroes and ones seen and provides access to partial results
/// of the overall probability estimates computed in the context
/// tree.
struct Node {
  /// Children.
  zero_child: Option<Box<Node>>,
  one_child: Option<Box<Node>>,

  /// Frequencies.
  zeroes: u64,
  ones: u64,

  /// The log2 of the probability of a sequence with self.zeroes zeroes
  /// and self.ones one. Computed using the Krichevsky Trofimov
  /// estimator algorithm.
  log_kt_prob: f64,

  /// The log2 of the weighted block probability of the current sequence.
  /// Funtion of self.zeroes and self.ones and the values recursively
  /// provided by the children of this node (if any), lazily recomputed.
  log_weighted_prob: f64,
}

impl Node {
  /// Creates a node and all its children recursively until the provided
  /// depth is reached.
  pub fn create_root(target_depth: usize, current_depth: usize) -> Self {
    if target_depth == current_depth {
      return Node::new(None, None);  // Leaf.
    } else {
      return Node::new(
          Some(Box::new(Node::create_root(target_depth, current_depth + 1))),
          Some(Box::new(Node::create_root(target_depth, current_depth + 1))));
    }
  }

  fn new(zero_child: Option<Box<Node>>, one_child: Option<Box<Node>>) -> Self {
    // Context trees are complete trees (either both children or no children).
    assert!(zero_child.is_none() == one_child.is_none());

    return Node {
      zero_child: zero_child,
      one_child: one_child,
      zeroes: 0,
      ones: 0,
      log_kt_prob: 0.0,
      log_weighted_prob: 0.0,
    };
  }

  pub fn log_weighted_prob(&self) -> f64 {
    self.log_weighted_prob
  }

  /// Updates the state of the node based on the value of the new bit.
  /// Assumes that child nodes (if any) have already been updated.
  pub fn update(&mut self, bit: Bit) {
    match bit {
      Bit::Zero => self.zeroes += 1,
      Bit::One => self.ones += 1,
    }

    // Old value + 0.5 is the same as new value - 0.5.
    let first_summand = match bit {
      Bit::Zero => self.zeroes as f64 - 0.5,
      Bit::One => self.ones as f64 - 0.5,
    }.log2();
    let norm_summand = ((self.zeroes + self.ones) as f64).log2();

    self.log_kt_prob = first_summand - norm_summand + self.log_kt_prob;
    self.update_weighted_prob();
  }

  /// Corresponds to undoing one update for the specified bit value.
  /// Assumes that child nodes (if any) have already been reverted.
  pub fn revert(&mut self, bit: Bit) {
    match bit {
      Bit::Zero => self.zeroes -= 1,
      Bit::One => self.ones -= 1,
    }
    // TODO(dinowernli): Update kt probs.
    self.update_weighted_prob();
  }

  /// Updates the value of log_weighted_prob. Assumes that log_kt_prob
  /// as well as all values in all children (if any) are up-to-date.
  fn update_weighted_prob(&mut self) {
    if self.is_leaf() {
      self.log_weighted_prob = self.log_kt_prob;
    }

    let log_weighted_probs =
        self.zero_child.as_ref().unwrap().log_weighted_prob() +
        self.one_child.as_ref().unwrap().log_weighted_prob();
    let exponent = log_weighted_probs - self.log_kt_prob;
    let power = exponent.exp2();

    // The recursive term should be log2(1 + 2^exponent). If the exponent is
    // large enough, 2^exponent is inf. In this case, we approximate
    // log2(1 + 2^exponent) by log2(2^exponent) = exponent.
    let inf = 1.0f64 / 0.0f64;
    let recursive_term =
        if power == inf { exponent } else { (power + 1.0).log2() };

    self.log_weighted_prob = self.log_kt_prob + recursive_term - 1.0;
  }

  /// Returns the number of nodes in the subtree rooted at this node.
  fn size(&self) -> usize {
    1 + match self.zero_child {
      None => 0,
      Some(ref node) => node.size(),
    } + match self.one_child {
      None => 0,
      Some(ref node) => node.size(),
    }
  }

  fn is_leaf(&self) -> bool {
    // By construction we have either both children or none.
    return self.zero_child.is_none();
  }
}
