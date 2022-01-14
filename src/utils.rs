#[macro_export]
macro_rules! clock {
  ($x: stmt) => {
    let start = std::time::Instant::now();
    $x
    let elapsed_sec = std::time::Instant::now()
      .duration_since(start)
      .as_secs_f32();
    println!("Total: {:.10}", elapsed_sec);
  };
}
