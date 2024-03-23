use std::marker::PhantomData;

pub struct Point {
  x: i32,
  y: i32,
}

pub struct PointBuilder<S> {
  x: Option<i32>,
  y: Option<i32>,
  s : PhantomData<S>
}

pub struct XY {}
pub struct X {}
pub struct Y {}
pub struct Empty {}

impl PointBuilder<Empty> {
  pub fn new() -> Self {
    PointBuilder { x: None, y : None, s : PhantomData }
  }
  pub fn y(self, y: i32) -> PointBuilder<Y> {
    PointBuilder { x : self.x , y : Some(y), s : PhantomData }
  }
  pub fn x(self, x: i32) -> PointBuilder<X> {
    PointBuilder { x : Some(x) , y : self.y, s : PhantomData }
  }
}

impl PointBuilder<X> {
  pub fn y(self, y: i32) -> PointBuilder<XY> {
    PointBuilder { x : self.x , y : Some(y), s : PhantomData }
  }
}

impl PointBuilder<Y> {
  pub fn x(self, x: i32) -> PointBuilder<XY> {
    PointBuilder { x : Some(x) , y : self.y, s : PhantomData }
  }
}

impl PointBuilder<XY> {
  pub fn build(self) -> Point {
    Point { x : self.x.unwrap(), y : self.y.unwrap() }
  }
}

#[test]
fn works_krok2() -> Result<(), String> {
  let point2 = PointBuilder::new()
    .x(0)
    .y(1)
    .build();

  let point1 = PointBuilder::new()
    .y(1)
    .x(0)
    .build();
  println!("{} {}", point1.x, point2.y);
  Ok(())
}

#[test]
fn doesnt_compile_krok2() {
  let t = trybuild::TestCases::new();
  t.compile_fail("tests/cant_compile/unspecified_field_krok2.rs");
}