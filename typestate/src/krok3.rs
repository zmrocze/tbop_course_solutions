pub struct Point {
  x: i32,
  y: i32,
  z: i32,
}

pub struct PointBuilder<X, Y, Z> {
  x: X,
  y: Y, 
  z: Z,
}

impl<> PointBuilder<(), (), ()> {
  pub fn new() -> Self {
    PointBuilder { x: (), y: (), z: () }
  }
}

impl<Y, Z> PointBuilder<(), Y, Z> {
  pub fn x(self, x: i32) -> PointBuilder<i32, Y, Z> {
    PointBuilder { x , y : self.y, z : self.z }
  }
}

impl<X, Z> PointBuilder<X, (), Z> {
  pub fn y(self, y: i32) -> PointBuilder<X, i32, Z> {
    PointBuilder { x : self.x , y : y, z : self.z }
  }
}

impl<X, Y> PointBuilder<X, Y, ()> {
  pub fn z(self, z: i32) -> PointBuilder<X, Y, i32> {
    PointBuilder { x :self.x, y : self.y, z }
  }
}

impl PointBuilder<i32, i32, i32> {
  pub fn build(self) -> Point {
    Point { x : self.x, y : self.y , z : self.z }
  }
}

#[test]
fn cant_compile() {
  let t = trybuild::TestCases::new();
  t.compile_fail("tests/cant_compile/*.rs");
  t.compile_fail("tests/cant_compile/*.rs");
}

#[test]
fn works() {
  let point2 = PointBuilder::new()
    .x(0)
    .y(1)
    .z(2)
    .build();

  let point1 = PointBuilder::new()
    .y(1)
    .x(0)
    .z(2)
    .build();
  println!("Hello, world! {} {}", point1.x, point2.y);
}

fn main () {

}