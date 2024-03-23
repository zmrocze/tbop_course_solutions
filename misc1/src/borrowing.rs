
pub enum Node {
  Inner(InnerNode),
  Leaf(LeafNode),
}
struct InnerNode {
  value: i32,
  left: Box<Node>,
  right: Box<Node>,
}
struct LeafNode {
  value: i32,
}

trait Visitor {
  fn visit_inner(&mut self, node: &InnerNode);
  fn visit_leaf(&mut self, node: &LeafNode);

  fn visit_node(&mut self, node: &Node) {
    match node {
      Node::Inner(node) => self.visit_inner(node),
      Node::Leaf(node) => self.visit_leaf(node),
    }
  }
}

impl Node {
  fn accept<V: Visitor>(&self, visitor: &mut V) {
      match self {
          Node::Inner(inner) => visitor.visit_inner(inner),
          Node::Leaf(leaf) => visitor.visit_leaf(leaf),
      }
  }
}

struct TreePrinter {
  ident: usize,
}
impl Visitor for TreePrinter {
  fn visit_inner(&mut self, node: &InnerNode) {
    let InnerNode { value, left, right } = node;
    println!("{}Inner node: {}", std::iter::repeat(' ').take(self.ident).collect::<String>(), value);
    self.ident += 1;
    left.accept(self);
    right.accept(self);
    self.ident -= 1;
  }
  fn visit_leaf(&mut self, node: &LeafNode) {
    println!("{}Leaf node: {}", std::iter::repeat(' ').take(self.ident).collect::<String>(), node.value);
  }
}

trait Visitor2 {
  fn visit_inner(self, node: &InnerNode);
  fn visit_leaf(self, node: &LeafNode);
}

impl Node {
  fn accept2<V: Visitor2>(&self, visitor: V) {
      match self {
          Node::Inner(inner) => visitor.visit_inner(inner),
          Node::Leaf(leaf) => visitor.visit_leaf(leaf),
      }
  }
}

impl Visitor2 for &mut TreePrinter {
  fn visit_inner(self, node: &InnerNode) {
    let InnerNode { value, left, right } = node;
    println!("{}Inner node: {}", std::iter::repeat(' ').take(self.ident).collect::<String>(), value);
    self.ident += 1;
    left.accept(self);
    right.accept(self);
    self.ident -= 1;    
  }
  fn visit_leaf(self, node: &LeafNode) {
    println!("{}Leaf node: {}", std::iter::repeat(' ').take(self.ident).collect::<String>(), node.value);
  }
  // fn visit_inner(&mut self, node: &InnerNode) {
  //   let InnerNode { value, left, right } = node;
  //   println!("{}Inner node: {}", std::iter::repeat(' ').take(self.ident).collect::<String>(), value);
  //   self.ident += 1;
  //   left.accept(self);
  //   right.accept(self);
  //   self.ident -= 1;
  // }
  // fn visit_leaf(&mut self, node: &LeafNode) {
  //   println!("{}Leaf node: {}", std::iter::repeat(' ').take(self.ident).collect::<String>(), node.value);
  // }
}

#[test]
fn main() {
  let root = Node::Inner(InnerNode {
    value : 0, left : Box::new( Node::Leaf(LeafNode { value : 1}) ),
    right : Box::new( Node::Leaf(LeafNode { value : 2}) )
  });
  root.accept(&mut TreePrinter { ident : 0 });
  root.accept2(&mut TreePrinter { ident : 0 });
  // działa o.O
}