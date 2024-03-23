
#[derive(PartialEq, Debug)]
struct Graph {
    name: &'static str,
    nodes: Vec<Node>,
  }

#[derive(PartialEq, Debug)]
struct Node {
  neighbors: Vec<usize>,
}  

impl Node {
  pub fn new() -> Self {
    Node { neighbors : Vec::new() }
  }
  pub fn add_neighbour(&mut self, v: usize) {
    self.neighbors.push(v);
  }
}

impl Graph {
  pub fn add_edge(&mut self , u: usize, v: usize) {
    self.nodes[u].add_neighbour(v);
  }
}

macro_rules! graph {
  ($name:expr, directed, $n:expr, $(($u:expr, $v:expr)),* ) => {
    { 
      let mut g = Graph { name : $name, nodes : std::iter::repeat_with(Node::new).take($n).collect() };
      $( g.add_edge($u, $v); )*
      g
    }
  };
  ($name:expr, undirected, $n:expr, $(($u:expr, $v:expr)),* ) => {
    { 
      let mut g = Graph { name : $name, nodes : std::iter::repeat_with(Node::new).take($n).collect() };
      $( {g.add_edge($u, $v); g.add_edge($v, $u)} )*
      g
    }
  };
}

#[test]
fn test1() {
  // // Poniższe wywołanie powinno zbudować graf reprezentujący nieskierowaną krawędź.
  let g1 : Graph = graph!(
    "Edge", // nazwa grafu
    undirected, // krawędzie są nieskierowane
    2, // ilość wierzchołków
    (1 as usize, 0 as usize) // lista krawędzi
  );

  // // Poniższe wywołanie powinno zbudować skierowany cykl na 4 wierzchołkach.
  let g2 : Graph = graph!(
    "Cycle",
    directed,
    4,
    (1, 2), (2, 3), (3, 0), (0, 1)
  );
}

#[test]
fn test2() {
  assert_eq!(graph!("Empty", directed, 0,), Graph { name: "Empty", nodes: vec![] });
}

