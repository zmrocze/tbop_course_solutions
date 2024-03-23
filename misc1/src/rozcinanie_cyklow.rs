use std::{cell::RefCell, collections::{hash_map::DefaultHasher, HashSet}, rc::{Rc, Weak}};

use std::hash::{Hash, Hasher};
use std::ptr;

#[test]
fn experiment() {
  // experiment if references hash to the same thing, thats not a comprehensive experiment
  let five = 5;
  let five_ref = &five;
  let fiv_ref2 = &five;

  let mut hasher = DefaultHasher::new();
  ptr::hash(five_ref, &mut hasher);
  let actual = hasher.finish();

  let mut hasher = DefaultHasher::new();
  (five_ref as *const i32).hash(&mut hasher);
  let expected = hasher.finish();
  
  assert_eq!(actual, expected);

  let mut hasher2 = DefaultHasher::new();
  ptr::hash(fiv_ref2, &mut hasher2);
  let actual2 = hasher2.finish();

  let mut hasher2 = DefaultHasher::new();
  (fiv_ref2 as *const i32).hash(&mut hasher2);
  let expected2 = hasher2.finish();
  assert_eq!(actual2, expected2);
  assert_eq!(actual, actual2);
}

#[derive(Clone, Eq, PartialEq)]
struct Node {
    value: &'static str,
    hard_links: RefCell<Vec<Rc<Node>>>,
    weak_links: RefCell<Vec<Weak<Node>>>,
}

impl Node {
    pub fn new(value: &'static str) -> Rc<Node> {
        Rc::new(Node {
            value,
            hard_links: RefCell::new(vec![]),
            weak_links: RefCell::new(vec![]),
        })
    }

    pub fn add_link(&self, node: Rc<Node>) {
        self.hard_links.borrow_mut().push(node);
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)?;
        for link in self.hard_links.borrow().iter() {
            write!(f, "{:?}", link)?;
        }
        Ok(())
    }
}

fn acycle(root: Rc<Node>) -> () {
  let mut vis: HashSet<&Node> =HashSet::new();

  fn dfs(visited: &mut std::collections::HashSet<& Node >, x : Rc<Node> ) {
    visited.insert()
  }

  dfs(& mut vis, root)
}

#[test]
fn main() {
  let s1 = Node::new("word ");

  let s2 = Node::new("the ");
  s2.add_link(s1.clone());

  let s3 = Node::new("is ");
  s3.add_link(s2.clone());

  let s4 = Node::new("bird ");
  s4.add_link(s3.clone());

  let s5 = Node::new("the ");
  s5.add_link(s4.clone());

  println!("{:?}", s5);

  acycle(s1.clone());

  s1.add_link(s5.clone());
  println!("{:?}", s5);
}



