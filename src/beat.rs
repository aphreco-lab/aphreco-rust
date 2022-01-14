#[macro_export]
macro_rules! beat {
  // macro to create beat array or a beat containing Decimals.
  // beat!(start, stop, interval) : three f64 values, comma separated.
  // => return 1d array of [start, stop, interval] of Decimal.
  ($start: expr, $stop: expr, $step: expr) => {
    [
      Decimal::from_str(&$start.to_string()).unwrap(),
      Decimal::from_str(&$stop.to_string()).unwrap(),
      Decimal::from_str(&$step.to_string()).unwrap(),
    ]
  };
}
