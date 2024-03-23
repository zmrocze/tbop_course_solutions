use typestate::krok3::PointBuilder;

fn unspecified_field() {
  let point = PointBuilder::new()
  .y(1)
  .build();
}

fn main() {}