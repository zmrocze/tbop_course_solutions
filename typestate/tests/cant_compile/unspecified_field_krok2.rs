use typestate::krok2::PointBuilder;

fn doesnt_compile_krok2() {
  let point  = PointBuilder::new()
    .x(1)
    .build();
}

fn main() {}