use typestate::krok3::PointBuilder;

fn field_specified_twice() {
  let point = PointBuilder::new()
  .x(1)
  .x(1)
  .build();
}

fn main() {}