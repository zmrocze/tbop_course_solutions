struct Point {
  x: i32,
  y: i32,
}

struct PointBuilder {
  x: Option<i32>,
  y: Option<i32>
}

impl PointBuilder {
  pub fn new() -> Self {
    PointBuilder { x: None, y : None }
  }
}

impl PointBuilder {
  pub fn y(self, y: i32) -> Result<PointBuilder, String> {
    match self.y {
      Some(_) => {Err("Field y is already set!".to_string())},
      None => {Ok(PointBuilder { x : self.x , y : Some(y) })},
    }
  }
  pub fn x(self, x: i32) -> Result<PointBuilder, String> {
    match self.x {
      Some(_) => {Err("Field x is already set!".to_string())},
      None => {Ok(PointBuilder { x : Some(x) , y : self.y })},
    }
  }
}

impl PointBuilder {
  pub fn build(self) -> Result<Point, String> {
    let y = self.y.ok_or("Field y not set!".to_string())?;
    let x = self.x.ok_or("Field x not set!".to_string())?;
    Ok(Point { x, y })
  }
}

#[test]
fn works_krok1() -> Result<(), String> {
  let point2 = PointBuilder::new()
    .x(0)?
    .y(1)?
    .build()?;

  let point1 = PointBuilder::new()
    .y(1)?
    .x(0)?
    .build()?;
  println!("{} {}", point1.x, point2.y);
  Ok(())
}
